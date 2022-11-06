use std::hash::{Hash, Hasher};
use xml::name::OwnedName;
use xml::attribute::OwnedAttribute;

use crate::filter::Filter;

#[derive(Debug, Clone)]
pub struct Tag {
    key: String,
    value: String,
}

impl Tag{
    pub fn new() -> Self {
        Tag {
            key: String::new(),
            value: String::new(),
        }
    }

    pub fn with_attributes(attributes: &Vec<OwnedAttribute>) -> Self {
        Tag {
            key: attributes[0].value.clone(),
            value: attributes[1].value.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Bbox {
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
    pub top: f64,
}

impl Bbox {
    pub fn new() -> Self {
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
        let mut way = Way::new();

        for elem in attributes {
            match &*elem.name.local_name {
                "id" => way.id = elem.value.parse::<usize>().unwrap(),
                "version" => way.version = elem.value.parse::<usize>().unwrap(),
                "timestamp" => way.timestamp = elem.value,
                "changeset" => way.changeset = elem.value.parse::<usize>().unwrap(),
                "uid" => way.uid = elem.value.parse::<usize>().unwrap(),
                "user" => way.user = elem.value,
                _ => {}
            }
        }

        way 

    }

    pub fn add_tag(&mut self, attributes: Vec<OwnedAttribute>) {
        let tag = Tag::with_attributes(&attributes);
        self.tags.push(tag);
    }

    pub fn tag_valid(&self, filter: &Filter) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone)]
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
        let mut relation = Relation::new();

        for elem in attributes {
            match &*elem.name.local_name {
                "id" => relation.id = elem.value.parse::<usize>().unwrap(),
                "version" => relation.version = elem.value.parse::<usize>().unwrap(),
                "timestamp" => relation.timestamp = elem.value,
                "changeset" => relation.changeset = elem.value.parse::<usize>().unwrap(),
                "uid" => relation.uid = elem.value.parse::<usize>().unwrap(),
                "user" => relation.user = elem.value,
                _ => {}
            }
        }

        relation
    }

    pub fn add_tag(&mut self, attributes: Vec<OwnedAttribute>) {
        let tag = Tag::with_attributes(&attributes);
        self.tags.push(tag);
    }

    pub fn add_member(&mut self, attributes: Vec<OwnedAttribute>){
        let member = Member::with_attributes(attributes);
        self.members.push(member);
    }
}

#[derive(Clone, Debug)]
pub struct Member {
    pub member_type: OsmElement,
    pub ref_id: usize,
    pub role: String,
}

impl Member {
    pub fn new() -> Self {
        Member { member_type: OsmElement::Node, ref_id: 0 as usize, role: String::new() }
    }

    pub fn with_attributes(attributes: Vec<OwnedAttribute>) -> Self {
        let mut member = Member::new();

        for elem in attributes {
            match &*elem.name.local_name {
                "type" => {match &*elem.value {
                    "node" => member.member_type = OsmElement::Node,
                    "way" => member.member_type = OsmElement::Way,
                    "relation" => member.member_type = OsmElement::Relation,
                    _ => {}
                }}
                "ref" => member.ref_id = elem.value.parse::<usize>().unwrap(),
                "role" => member.role = elem.value,
                _ => {}
            }
        }

        member
    }
}
#[derive(Debug, Clone)]
pub enum OsmElement {
    Node,
    Way,
    Relation
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
            user: String::new(),
            tags: Vec::<Tag>::new(),
        }
    }

    pub fn with_attributes(attributes: Vec<OwnedAttribute>) -> Self {
        let mut node = Node::new();
        for elem in attributes {
            match &*elem.name.local_name {
                "id" => node.id = elem.value.parse::<usize>().unwrap(),
                "lat" => node.lat = elem.value.parse::<f64>().unwrap(),
                "lon" => node.lon = elem.value.parse::<f64>().unwrap(),
                "version" => node.version = elem.value.parse::<usize>().unwrap(),
                "timestamp" => node.timestamp = elem.value,
                "changeset" => node.changeset = elem.value.parse::<usize>().unwrap(),
                "uid" => node.uid = elem.value.parse::<usize>().unwrap(),
                "user" => node.user = elem.value,
                _ => {}
            }
        }
        node
    }

    pub fn add_tag(&mut self, attributes: Vec<OwnedAttribute>) {
        let tag = Tag::with_attributes(&attributes);
        self.tags.push(tag);
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

#[derive(Copy, Clone)]
pub enum NetworkType {
    Walk,
    Bike,
    Drive,
    DriveService,
    AllPrivate,
    All,
}