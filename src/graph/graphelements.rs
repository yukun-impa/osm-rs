use crate::osmelements::{Node as OSMNode, Way};
use petgraph::graph::NodeIndex;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Default, Copy)]
pub struct Node {
    pub id: usize,
    pub lat: f64,
    pub lon: f64,
}


impl Node {
    pub fn new() -> Self {
        Node {
            id: 0 as usize,
            lat: 0.0,
            lon: 0.0,
        }
    }

    pub fn from_osmnode(osmnode: &OSMNode) -> Self {
        Node {
            id: osmnode.id,
            lat: osmnode.lat,
            lon: osmnode.lon,
        }
    }
}


impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Link {
    pub id: usize,
    pub from: NodeIndex,
    pub to: NodeIndex,
    pub length: f64,
}

impl Link {
    pub fn new() {
        todo!()
    }

    pub fn from_way(way: &Way) -> Vec<Self> {
        let one_way = way.is_one_way();
        todo!()
    }

}

