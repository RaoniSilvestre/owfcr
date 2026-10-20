use crate::types::{Cables, FastIntersectionTester, Graph, Problem, WindFarmNode};
use color_eyre::eyre::{ContextCompat, Result};
use std::{
    fs::{self},
    path::Path,
    sync::Arc,
};
use tracing::info;

pub fn load(path: &Path) -> Result<Problem> {
    let instance_name = get_instance_name(path);

    info!("Inicializando problema {:?}", instance_name);

    let cables = load_cables(path)?;
    let nodes = load_nodes(path)?;

    let graph = Graph::new(nodes);
    let colisions = Arc::new(FastIntersectionTester::new(&graph));

    info!("Problema {:?} inicializado.", instance_name);

    Ok(Problem {
        instance_name,
        colisions,
        graph,
        cables,
    })
}

fn load_cables(path: &Path) -> Result<Cables> {
    let cable_path_name = path.with_extension("cable");
    let cable_path = Path::new(&cable_path_name);
    let cable_content = fs::read_to_string(cable_path)?;

    Cables::try_from(cable_content)
}

fn load_nodes(path: &Path) -> Result<Vec<WindFarmNode>> {
    let content = fs::read_to_string(path)?;

    let mut nodes: Vec<WindFarmNode> = Vec::new();
    for line in content.lines() {
        let mut line = line.split_whitespace();
        let x: f64 = line.next().context("X wasnt found")?.parse()?;
        let y: f64 = line.next().context("Y wasnt found")?.parse()?;
        let w: i8 = line.next().context("Weight wasnt found")?.parse()?;

        nodes.push(WindFarmNode::new(x, y, w));
    }

    Ok(nodes)
}

fn get_instance_name(path: &Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_owned()
}
