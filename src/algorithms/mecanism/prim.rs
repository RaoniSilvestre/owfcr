use crate::{
    types::{Idx, Problem},
    utils,
};

/// Constrói a MST enraizada na subestação, conectando `edges`.
///
/// Caso particular de [`prim_seeded`] com máscara de cruzamento vazia.
pub fn prim(problem: &Problem, edges: &[Idx], parents: &mut [Idx]) {
    prim_seeded(problem, edges, parents, &[]);
}

/// Constrói uma MST conectando `group` à subestação, evitando cruzar `seed_mask`.
///
/// As arestas de `seed_mask` (já presentes na solução) não são alteradas e não são cruzadas.
/// Se nenhuma aresta sem cruzamento existir, usa a de menor distância (aceitando a penalidade),
/// em vez de panificar.
pub fn prim_seeded(problem: &Problem, group: &[Idx], parents: &mut [Idx], seed_mask: &[u64]) {
    let substation = problem.graph.substation();

    let mut nodes = group.to_vec();
    nodes.push(substation);

    let mut in_tree = vec![false; nodes.len()];
    in_tree[nodes.len() - 1] = true; // subestação já na árvore

    let mut active_mask = vec![0u64; problem.colisions.blocks_per_row];
    debug_assert!(seed_mask.len() <= active_mask.len());
    active_mask[..seed_mask.len()].copy_from_slice(seed_mask);

    for _ in 0..group.len() {
        let mut min_edge: Option<(usize, usize)> = None;
        let mut min_distance = f64::INFINITY;

        let mut fallback: Option<(usize, usize)> = None;
        let mut fallback_distance = f64::INFINITY;

        for i in 0..nodes.len() {
            if in_tree[i] {
                for j in 0..nodes.len() {
                    if !in_tree[j] {
                        let distance = problem.graph[nodes[i]][nodes[j]];

                        if distance < fallback_distance {
                            fallback_distance = distance;
                            fallback = Some((i, j));
                        }

                        if distance < min_distance
                            && !problem
                                .colisions
                                .any_crossings(nodes[i], nodes[j], &active_mask)
                        {
                            min_distance = distance;
                            min_edge = Some((i, j));
                        }
                    }
                }
            }
        }

        let (u, v) = min_edge
            .or(fallback)
            .expect("MIN edge não encontrada no prim_seeded.");

        in_tree[v] = true;
        parents[nodes[v]] = nodes[u];

        let eid = utils::edge_id(nodes[v], nodes[u]);
        active_mask[eid / 64] |= 1 << (eid % 64);
    }
}
