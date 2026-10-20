use crate::types::{Angle, Graph, Idx};

/// Sort the turbines according to the angle relative to the substation.
///
/// Returns a vector of `(node index, angle)` sorted by the angle.
pub fn sort_turbines_by_angle(graph: &Graph) -> Vec<(Idx, Angle)> {
    let substation = graph.substation();
    let substation_node = graph.node(substation);

    let mut turbines: Vec<(Idx, Angle)> = graph
        .nodes()
        .filter(|&idx| idx != substation)
        .map(|idx| {
            let node = graph.node(idx);
            let angle = (node.y - substation_node.y).atan2(node.x - substation_node.x);
            (idx, angle)
        })
        .collect();

    turbines.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("Comparação não deveria falhar..."));

    turbines
}
