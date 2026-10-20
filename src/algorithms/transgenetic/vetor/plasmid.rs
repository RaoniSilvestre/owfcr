use super::TransgeneticVector;
use crate::{
    algorithms::{mecanism, transgenetic::vetor::PartialSolution},
    types::{Idx, Problem, Solution},
};
use rand::{RngExt, rngs::StdRng, seq::SliceRandom};

/// Precomputed genetic material from a donor solution: a connected set of substation branches.
///
/// The representation is sparse: only the genes that will be transferred.
pub struct Plasmid {
    pub genes: Vec<(Idx, Idx)>,
}

impl TransgeneticVector for Plasmid {
    fn transcribe(
        &self,
        problem: &Problem,
        receptor: &Solution,
        rng: &mut StdRng,
        iters: &mut usize,
    ) -> Solution {
        let substation = problem.graph.substation();
        let mut partial: PartialSolution = receptor.into();
        partial.apply_genes(&self.genes, substation);

        let mut solution = super::close_partial(problem, partial, rng);
        *iters += 1;

        *iters += mecanism::local_search(problem, &mut solution);

        solution
    }
}

impl Plasmid {
    pub fn generate_bank(
        problem: &Problem,
        elite: &[Solution],
        bank_size: usize,
        size_range: (f64, f64),
        rng: &mut StdRng,
    ) -> Vec<Self> {
        let substation = problem.graph.substation();
        let mut bank = Vec::with_capacity(bank_size);

        let mut i = 0;
        while bank.len() < bank_size {
            let donor = &elite[i % elite.len()];
            bank.push(extract_plasmid(donor, size_range, substation, rng));
            i += 1;
        }

        bank
    }
}

/// Builds a plasmid from a donor: samples `k` substation-rooted branches, where
/// `k` is proportional to a random size drawn inside [`min`, `max`] of the solution.
#[allow(clippy::cast_sign_loss)]
fn extract_plasmid(
    donor: &Solution,
    (min, max): (f64, f64),
    substation: Idx,
    rng: &mut StdRng,
) -> Plasmid {
    let branches = substation_branches(donor, substation);
    let mut genes = Vec::new();

    let frac = rng.random_range(min..max);
    let k = ((frac * branches.len() as f64).round() as usize).clamp(1, branches.len());

    let mut order: Vec<usize> = (0..branches.len()).collect();
    order.shuffle(rng);

    for &bi in order.iter().take(k) {
        for &node in &branches[bi].1 {
            genes.push((node, donor.parents[node]));
        }
    }

    Plasmid { genes }
}

/// Substation-rooted branches of a solution: each direct child of the substation with its subtree.
///
/// If the parents is [0, 0, 1, 0, 3], then and the substation being 0, then we will have
///
/// [
/// (1, [1, 2]),
/// (3, [3, 4]),
/// ]
fn substation_branches(solution: &Solution, substation: Idx) -> Vec<(Idx, Vec<Idx>)> {
    let n = solution.parents.len();

    let mut children = vec![Vec::new(); n];

    // v is an idx and p is the parent of that idx. so it refers to an edge in the current solution.
    // avoiding the substation, for every parent[x], adds x to the child[p] vec.
    //
    // [
    //  [],       // 0 (subestação não guarda filhos aqui)
    //  [2],      // 1 é pai de 2
    //  [],       // 2 não tem filhos
    //  [4],      // 3 é pai de 4
    //  [],       // 4 não tem filhos
    // ]
    for (v, &p) in solution.parents.iter().enumerate() {
        if v != substation && p != substation {
            children[p].push(v);
        }
    }

    let mut branches = Vec::new();
    for (v, &p) in solution.parents.iter().enumerate() {
        // Se o nó não é a subestação e seu pai direto É a subestação:
        if v != substation && p == substation {
            // Coleta todos os nós da sub-árvore a partir desse filho direto
            let nodes = subtree_nodes_from(&children, v);
            branches.push((v, nodes));
        }
    }

    branches
}

/// Coleta todos os nós pertencentes à sub-árvore enraizada em `root` (inclusive),
/// utilizando uma travessia iterativa baseada em pilha (DFS - Depth-First Search).
///
/// # Parâmetros
/// - `children`: Lista de adjacência onde o índice representa um pai e o valor é um vetor com seus filhos diretos.
/// - `root`: O nó raiz a partir do qual a sub-árvore será percorrida.
///
/// # Retorno
/// Um vetor contendo os índices de todos os nós encontrados na sub-árvore.
fn subtree_nodes_from(children: &[Vec<Idx>], root: Idx) -> Vec<Idx> {
    let mut nodes = Vec::new();
    let mut stack = vec![root];

    while let Some(u) = stack.pop() {
        nodes.push(u);
        stack.extend(children[u].iter().copied());
    }

    nodes
}
