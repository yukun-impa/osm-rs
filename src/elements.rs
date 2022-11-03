use std::hash::{Hash, Hasher};
use xml::name::OwnedName;
use xml::attribute::OwnedAttribute;

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
    pub id: usize,
    pub version: usize,
    pub timestamp: String,
    pub changeset: usize,
    pub uid: usize,
    pub user: String,
    pub nodes: Vec<usize>,
    pub tags: Vec<Tag>
}
impl Way {
    pub(crate) fn new() -> Self {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Node {
    pub id: usize,
    pub lat: f64,
    pub lon: f64,
    pub version: usize,
    pub timestamp: String,
    pub changeset: usize,
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
impl Relation {
    pub(crate) fn new() -> Self {
        todo!()
    }
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
    pub fn new() -> Self {
        Node {
            id: todo!(),
            lat: todo!(),
            lon: todo!(),
            version: todo!(),
            timestamp: todo!(),
            changeset: todo!(),
            uid: todo!(),
            user: todo!(),
            tags: todo!(),
        }
    }

    pub fn read(&mut self, name: OwnedName, attributes: Vec<xml::attribute::OwnedAttribute>) {
        for elem in attributes {
            match &elem.name {
                "id" => self.id = elem.value.parse::<usize>().unwrap(),
                "lat" => self.lat = elem.value.parse::<f64>().unwrap(),
                "lon" => self.lon = elem.value.parse::<f64>().unwrap(),
                "version" => self.version = elem.value.parse::<usize>().unwrap(),
                "timestamp" => self.timestamp = elem.value,
                "changeset" => self.changeset = elem.value.parse::<usize>().unwrap(),
                "uid" => self.uid = elem.value.parse::<usize>().unwrap(),
                "user" => self.user = elem.value
            }
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