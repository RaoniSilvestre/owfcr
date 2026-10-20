use std::collections::VecDeque;

use crate::{
    algorithms::mecanism,
    types::{Idx, Mask, Problem, Solution},
};
use rand::{rngs::StdRng, seq::SliceRandom};

mod plasmid;
mod sector;
mod transposson;

pub(super) use plasmid::Plasmid;
pub(super) use sector::SectorTransposon;
pub(super) use transposson::Transposon;

pub(super) trait TransgeneticVector {
    fn attack(&self, original: &Solution, modified: &Solution) -> bool {
        original.cost > modified.cost
    }
    fn transcribe(
        &self,
        problem: &Problem,
        solution: &Solution,
        rng: &mut StdRng,
        iters: &mut usize,
    ) -> Solution;
    fn transcribed(
        &self,
        problem: &Problem,
        solution: &Solution,
        rng: &mut StdRng,
        iters: &mut usize,
    ) -> Option<Solution> {
        let transcribed = self.transcribe(problem, solution, rng, iters);
        if self.attack(solution, &transcribed) {
            return Some(transcribed);
        }
        None
    }
}

/// State of an organism during the horizontal transfer.
///
/// `Some(parent)` means the node is connected; `None` means it is "open" and still needs to be
/// reconnected by the closure phase.
pub(super) struct PartialSolution {
    pub(super) parents: Vec<Option<Idx>>,
    pub(super) flows: Vec<i8>,
    pub(super) mask: Mask,
    pub(super) substation_connections: i32,
}

impl From<&Solution> for PartialSolution {
    fn from(value: &Solution) -> Self {
        Self {
            parents: value.parents.iter().map(|&p| Some(p)).collect(),
            flows: value.flows.clone(),
            mask: value.mask.clone(),
            substation_connections: value.substation_connections,
        }
    }
}

impl PartialSolution {
    /// Desprende `child` do pai atual, subtraindo o fluxo da subárvore no caminho antigo.
    fn detach(&mut self, child: Idx, substation: Idx) {
        let child_flow = self.flows[child];
        let mut parent_opt = self.parents[child];

        // Diminui o fluxo árvore acima.
        while let Some(parent) = parent_opt {
            if parent == substation {
                break;
            }
            self.flows[parent] -= child_flow;
            parent_opt = self.parents[parent];
        }

        // Desconecta o pai atual.
        if let Some(old) = self.parents[child].take() {
            self.mask.remove_nodes(child, old);
            if old == substation {
                self.substation_connections -= 1;
            }
        }
    }

    /// Liga `child` → `parent`, somando o fluxo no caminho conectado até a subestação.
    fn attach(&mut self, child: Idx, parent: Idx, substation: Idx) {
        let child_flow = self.flows[child];
        self.parents[child] = Some(parent);
        self.mask.add_nodes(child, parent);

        // Se for substação, aumenta o numero de conexões.
        if parent == substation {
            self.substation_connections += 1;
        }

        let mut x = parent;
        // Vai adicionando o fluxo do filho árvore acima.
        while x != substation {
            self.flows[x] += child_flow;
            let Some(p) = self.parents[x] else { break }; // pai flutuante: para aqui
            x = p;
        }
    }

    /// Corte + splice da região transferida.
    pub(super) fn apply_genes(&mut self, genes: &[(Idx, Idx)], substation: Idx) {
        // 1) Corte
        for &(child, _) in genes {
            self.detach(child, substation);
        }
        // 2) Splice (ordem do doador já é pré-ordem: pai antes do filho)
        for &(child, parent) in genes {
            if parent != child && !creates_cycle(&self.parents, child, parent, substation) {
                self.attach(child, parent, substation);
            }
        }
    }

    /// Destrói em batch: desliga todos os `nodes`, depois recalcula os fluxos em O(n).
    pub(super) fn destroy(&mut self, nodes: &[Idx], substation: Idx) {
        for &v in nodes {
            if let Some(old) = self.parents[v].take() {
                self.mask.remove_nodes(v, old);
                if old == substation {
                    self.substation_connections -= 1;
                }
            }
        }
        self.recompute_flows();
    }

    fn recompute_flows(&mut self) {
        let n = self.parents.len();
        let mut child_count = vec![0usize; n];
        for (v, p) in self.parents.iter().enumerate() {
            self.flows[v] = 1;
            if let Some(p) = p
                && *p != v
            {
                child_count[*p] += 1;
            }
        }
        let mut queue: VecDeque<Idx> = (0..n).filter(|&v| child_count[v] == 0).collect();
        while let Some(u) = queue.pop_front() {
            let Some(p) = self.parents[u] else { continue };
            if p == u {
                continue;
            }
            self.flows[p] += self.flows[u];
            child_count[p] -= 1;
            if child_count[p] == 0 {
                queue.push_back(p);
            }
        }
    }
}

pub(super) fn close_partial(
    problem: &Problem,
    mut partial: PartialSolution,
    rng: &mut StdRng,
) -> Solution {
    let graph = &problem.graph;
    let n = graph.node_count();
    let substation = graph.substation();

    let mut open: Vec<Idx> = (0..n)
        .filter(|&v| v != substation && partial.parents[v].is_none())
        .collect();

    open.shuffle(rng);

    for v in open {
        let c = best_parent(problem, &partial, v);
        partial.attach(v, c, substation);
    }

    let complete: Vec<Idx> = partial
        .parents
        .iter()
        .map(|p| p.unwrap_or(substation))
        .collect();

    let mut solution = Solution::from_parents(complete, problem);
    mecanism::evaluate(problem, &mut solution);
    debug_assert!(mecanism::is_tree(&solution, problem));
    solution
}

fn best_parent(problem: &Problem, partial: &PartialSolution, v: Idx) -> Idx {
    let graph = &problem.graph;
    let substation = graph.substation();
    let flow_v = partial.flows[v];
    let limit = graph.substation_limit();

    let mut best = substation;
    let mut best_delta = f64::MAX;

    for c in graph.nodes() {
        if c == v || !reaches(&partial.parents, c, substation) {
            continue;
        }
        if creates_cycle(&partial.parents, v, c, substation) {
            continue;
        }

        // Custo do cabo a ser adicionado.
        let mut delta = problem.cables.get_cable_cost(flow_v, graph[v][c]);

        // Custo de cruzamentos adicionados.
        delta += f64::from(problem.colisions.count_crossings(v, c, &partial.mask)) * crate::PENALTY;

        // Custo de aumentar a quantidade de conexões a subestação.
        if c == substation {
            let conns = partial.substation_connections;
            let new_conns = conns + 1;
            delta += f64::from(
                (new_conns - i32::from(limit)).max(0) - (conns - i32::from(limit)).max(0),
            ) * crate::PENALTY;
        }

        // Custo árvore acima de conectar essa subárvore.
        let mut x = c;
        while x != substation {
            let p = partial.parents[x].expect("reaches garante Some");
            let d = graph[x][p];
            delta += problem.cables.get_cable_cost(partial.flows[x] + flow_v, d)
                - problem.cables.get_cable_cost(partial.flows[x], d);
            x = p;
        }

        if delta < best_delta {
            best_delta = delta;
            best = c;
        }
    }

    best
}

/// Verifies if `v` reaches the substation through the parent chain of a partial solution.
fn reaches(parents: &[Option<Idx>], mut node: Idx, substation: Idx) -> bool {
    let n = parents.len();
    let mut steps = 0;

    while node != substation {
        match parents[node] {
            Some(p) => node = p,
            None => return false,
        }

        steps += 1;
        if steps > n {
            return false;
        }
    }

    true
}

/// Checks if connecting `child` to `parent_candidate` would create a cycle on a partial solution.
///
/// `None` means the node has no parent yet, therefore the candidate chain cannot close a cycle.
fn creates_cycle(
    parents: &[Option<Idx>],
    child: Idx,
    parent_candidate: Idx,
    substation: Idx,
) -> bool {
    let mut parent_candidate = parent_candidate;

    if parent_candidate == substation {
        return false;
    }

    while parent_candidate != substation {
        if parent_candidate == child {
            return true;
        }

        if let Some(candidate) = parents[parent_candidate] {
            parent_candidate = candidate;
        } else {
            return false;
        }
    }

    false
}
