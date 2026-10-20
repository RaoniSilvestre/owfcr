//! Solution module.

use crate::{
    types::{Cost, FastIntersectionTester, Idx, Problem},
    utils::{self},
};
use std::{collections::VecDeque, ops::Deref, sync::Arc};

#[derive(Clone)]
pub struct Solution {
    pub cost: Cost,
    pub mask: Mask,
    pub parents: Vec<Idx>,
    pub flows: Vec<i8>,
    pub crossings: u32,
    pub substation_connections: i32,
    pub substation: Idx,
    colisions: Arc<FastIntersectionTester>,
}

impl Solution {
    #[must_use]
    pub fn new(problem: &Problem) -> Self {
        Self::from_parents(
            vec![problem.graph.substation(); problem.graph.node_count()],
            problem,
        )
    }

    #[must_use]
    pub fn from_parents(parents: Vec<Idx>, problem: &Problem) -> Self {
        let n = problem.graph.node_count();
        let mut s = Self {
            cost: 0.0,
            parents,
            flows: vec![0; n],
            crossings: 0,
            substation_connections: 0,
            mask: Mask::new(utils::block_size(&problem.graph)),
            substation: problem.graph.substation(),
            colisions: Arc::clone(&problem.colisions),
        };
        build_flows(&mut s.flows, &s.parents, s.substation, n);
        for x in problem.graph.nodes() {
            // mask + nº na subestação
            if x == s.substation {
                continue;
            }
            let p = s.parents[x];
            // Adiciona a aresta entre x e pai[x]
            s.mask.add_nodes(x, p);

            // se o pai for a subestação, adicione 1 na contagem de conectados.
            if p == s.substation {
                s.substation_connections += 1;
            }
        }
        let mut total = 0; // crossings (pares)

        // Para cada nó, conte os cruzamentos do nó com seu pai, depois divida por dois.
        for x in problem.graph.nodes() {
            if x == s.substation {
                continue;
            }
            total += s.colisions.count_crossings(x, s.parents[x], &s.mask);
        }
        // Tem que dividir por dois mesmo?
        s.crossings = total / 2;
        s
    }

    pub fn swap_edge(&mut self, child: Idx, new: Idx) -> Idx {
        let old = self.parents[child];
        if old == new {
            return old;
        }
        let fi = self.flows[child];

        let old_collisions = self.colisions.count_crossings(child, old, &self.mask);
        self.mask.remove_nodes(child, old);
        self.mask.add_nodes(child, new);
        let new_collisions = self.colisions.count_crossings(child, new, &self.mask);
        self.crossings =
            (self.crossings as i64 + new_collisions as i64 - old_collisions as i64) as u32;

        self.parents[child] = new;

        // flows: subtrai fi em old→sub, soma em new→sub (overlap cancela)
        let mut x = old;
        while x != self.substation {
            self.flows[x] -= fi;
            x = self.parents[x];
        }
        x = new;
        while x != self.substation {
            self.flows[x] += fi;
            x = self.parents[x];
        }

        // subestações
        if old == self.substation {
            self.substation_connections -= 1;
        }
        if new == self.substation {
            self.substation_connections += 1;
        }

        old
    }

    #[inline]
    #[must_use]
    pub fn has_edge(&self, u: Idx, v: Idx) -> bool {
        self.mask.contains_nodes(u, v)
    }

    pub fn set_cost(&mut self, cost: Cost) {
        self.cost = cost;
    }

    pub fn any_crossings(&self, u: Idx, v: Idx) -> bool {
        self.colisions.any_crossings(u, v, &self.mask)
    }

    pub fn count_crossings(&self, u: Idx, v: Idx) -> u32 {
        self.colisions.count_crossings(u, v, &self.mask)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mask {
    mask: Vec<u64>,
}

impl Mask {
    #[must_use]
    pub fn new(block_size: usize) -> Self {
        Mask {
            mask: vec![0; block_size],
        }
    }

    #[inline]
    pub fn add_nodes(&mut self, u: Idx, v: Idx) {
        let eid = utils::edge_id(u, v);
        self.mask[eid / 64] |= 1 << (eid % 64);
    }

    #[inline]
    pub fn remove_nodes(&mut self, u: Idx, v: Idx) {
        let eid = utils::edge_id(u, v);
        self.mask[eid / 64] &= !(1 << (eid % 64));
    }

    #[inline]
    #[must_use]
    pub fn contains_nodes(&self, u: Idx, v: Idx) -> bool {
        let eid = utils::edge_id(u, v);
        let block_idx = eid / 64;
        let bit_idx = eid % 64;

        if block_idx < self.mask.len() {
            (self.mask[block_idx] & (1 << bit_idx)) != 0
        } else {
            false
        }
    }
}

impl Deref for Mask {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        &self.mask
    }
}

fn build_flows(flows: &mut [i8], parents: &[Idx], substation: Idx, n: usize) {
    let mut child_count = vec![0usize; n];
    let mut queue = VecDeque::with_capacity(n);

    // Preenche quantos filhos cada nó tem.
    for x in 0..n {
        if x == substation {
            continue;
        }

        let pai_de_x = parents[x];

        // Se x é filho de px, então px tem +1 filho do que antes.
        child_count[pai_de_x] += 1;
        flows[x] = 1;
    }

    // Adiciona na filha todos os nós folha.
    for x in 0..n {
        if x != substation && child_count[x] == 0 {
            queue.push_back(x);
        }
    }

    // Para cada nó, iniciando das folhas e subindo, adicione o fluxo do filho ao fluxo do pai e
    // diminua a contagem de filhos do pai. Se o pai deixar de ser nó intermediário adicione-o a fila.
    while let Some(u) = queue.pop_front() {
        let p = parents[u];
        if p == substation {
            continue;
        }
        flows[p] += flows[u];
        child_count[p] -= 1;
        if child_count[p] == 0 {
            queue.push_back(p);
        }
    }
}
