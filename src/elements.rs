use std::hash::{Hash, Hasher};

pub struct Tag {
    key: String,
    value: String,
}
#[derive(Debug)]
pub struct Bbox {
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
    pub top: f64,
}

impl Bbox {
    fn new() -> Self {
        Bbox {
            left: 0.0,
            bottom: 0.0,
            right: 0.0,
            top: 0.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Way {
    pub id: usize
    pub version: usize,
    pub timestamp: String,
    pub changeset: usize,
    pub uid: usize,
    pub user: String,
    pub nodes: Vec<usize>,
    pub tags: Vec<Tag>
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Node {
    pub id: usize,
    pub lat: f64,
    pub lon: f64,
    pub version: usize,
    pub timestamp: String,
    pub changeset: usize
    pub uid: usize,
    pub user: String,
    pub tags: Vec<Tag>
}

pub struct Relation {
    pub id: usize,
    pub version: usize,
    pub timestamp: String,
    pub changeset: usize,
    pub uid: usize,
    pub user: String,
    pub members: Vec<Member>,
    pub tags: Vec<Tag>
}

pub struct Member {
    pub member_type: OsmElement,
    pub ref_id: usize,
    pub role: String,
}

enum OsmElement {
    Node,
    Way,
    Relation
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_id.hash(state);
    }
}

impl Node {
    pub fn new(id: usize, longitude: f64, latitude: f64) -> Self {
        Node {
            node_id: id,
            longitude,
            latitude,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Link {
    pub link_id: usize,
    pub ref_node_id: usize,
    pub non_ref_node_id: usize,
    pub length_m: f64,
}

impl Link {
    pub fn new(link_id: usize, ref_node_id: usize, non_ref_node_id: usize, length_m: f64) -> Self {
        Link {
            link_id,
            ref_node_id,
            non_ref_node_id,
            length_m,
        }
    }
}