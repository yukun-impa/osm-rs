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
    pub fn new() -> Self {
        Way {
            id: 0 as usize,
            version: 0 as usize,
            timestamp: String::new(),
            changeset: 0 as usize,
            uid: 0 as usize,
            user: String::new(),
            nodes: Vec::<usize>::new(),
            tags: Vec::<Tag>::new(),
        }
    }

    pub fn with_attributes(attributes: Vec<OwnedAttribute>) -> Self {
        let way = Way::new();

        for elem in attributes {
            match &elem.name {
                "id" => relation.id = elem.value.parse::<usize>().unwrap(),
                "version" => relation.version = elem.value.parse::<usize>().unwrap(),
                "timestamp" => relation.timestamp = elem.value,
                "changeset" => relation.changeset = elem.value.parse::<usize>().unwrap(),
                "uid" => relation.uid = elem.value.parse::<usize>().unwrap(),
                "user" => relation.user = elem.value
            }
        }

        way 

    }

    pub fn add_tag(&self, attributes: Vec<OwnedAttribute>) -> _ {
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
    
    pub fn new() -> Self {
        Relation {
            id: 0 as usize,
            version: 0 as usize,
            timestamp: String::new(),
            changeset: 0 as usize,
            uid: 0 as usize,
            user: String::new(),
            members: Vec::<Member>::new(),
            tags: Vec::<Tag>::new(),
        }
    }

    pub fn with_attributes(attributes: Vec<OwnedAttribute>) -> Self {
        let relation = Relation::new();

        for elem in attributes {
            match &elem.name {
                "id" => relation.id = elem.value.parse::<usize>().unwrap(),
                "version" => relation.version = elem.value.parse::<usize>().unwrap(),
                "timestamp" => relation.timestamp = elem.value,
                "changeset" => relation.changeset = elem.value.parse::<usize>().unwrap(),
                "uid" => relation.uid = elem.value.parse::<usize>().unwrap(),
                "user" => relation.user = elem.value
            }
        }

        relation
    }
    pub fn add_tag(&self, attributes: Vec<OwnedAttribute>) {
        todo!()
    }
}

pub struct Member {
    pub member_type: OsmElement,
    pub ref_id: usize,
    pub role: String,
}

impl Member {
    fn new() -> Self {
        Member { member_type: OsmElement::Node, ref_id: 0 as usize, role: String::new() }
    }

    fn with_attributes(attributes: Vec<OwnedAttribute>) -> Self {
        let member = Member::new();

        for elem in attributes {
            match &elem.name {
                "type" => {match &elem.value {
                    "node" => member.member_type = OsmElement::Node,
                    "way" => member.member_type = OsmElement::Way,
                    "relation" => member.member_type = OsmElement::Relation,
                }}
                "ref" => member.ref_id = elem.value.parse::<usize>().unwrap(),
                "role" => member.role = elem.value,

            }
        }

        member
    }
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
            id: 0 as usize,
            lat: 0.0,
            lon: 0.0,
            version: 0 as usize,
            timestamp: String::new(),
            changeset: 0 as usize,
            uid: 0 as usize,
            user: String::new,
            tags: Vec::<Tag>::new(),
        }
    }

    pub fn with_attributes(attributes: Vec<OwnedAttribute>) -> Self {
        let mut node = Node::new();
        for elem in attributes {
            match &elem.name {
                "id" => node.id = elem.value.parse::<usize>().unwrap(),
                "lat" => node.lat = elem.value.parse::<f64>().unwrap(),
                "lon" => node.lon = elem.value.parse::<f64>().unwrap(),
                "version" => node.version = elem.value.parse::<usize>().unwrap(),
                "timestamp" => node.timestamp = elem.value,
                "changeset" => node.changeset = elem.value.parse::<usize>().unwrap(),
                "uid" => node.uid = elem.value.parse::<usize>().unwrap(),
                "user" => node.user = elem.value
            }
        }
        node
    }

    pub fn add_tag(&mut self, attributes: Vec<OwnedAttribute>) {
        todo!()
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