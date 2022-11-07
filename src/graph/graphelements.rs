use crate::reader::osmelements::{Node as OSMNode, Way};
use map_3d::distance;
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
    pub from: usize,
    pub to: usize,
    pub length: f64,
}

impl Link {
    pub fn new() -> Self {
        todo!()
    }

    pub fn with_attributes(id: usize, from: usize, to: usize, length: f64) -> Self {
        Link {
            id,
            from,
            to,
            length,
        }
    }

    pub fn from_way(way: &Way, nodes: &Vec<OSMNode>) -> Vec<Self> {
        let mut links = Vec::<Self>::new();
        for i in 0..way.nodes.len() - 1 {
            let from = nodes.iter().find(|&node| node.id == way.nodes[i]).unwrap();
            let to = nodes
                .iter()
                .find(|&node| node.id == way.nodes[i + 1])
                .unwrap();
            let distance = distance((from.lat, from.lon), (to.lat, to.lon));
            let link = Link::with_attributes(way.id, from.id, to.id, distance);
            links.push(link)
        }
        links
    }
}
