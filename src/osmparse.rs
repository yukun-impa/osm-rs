use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use xml::reader::{EventReader, XmlEvent};
use crate::elements::{Tag, Bbox, Node, Way, Relation, Member};
pub struct OSM {
    bbox: Bbox,
    nodes: Vec<Node>,
    ways: Vec<Way>,
    relations: Vec<Relation>
}

impl OSM {
    fn new() -> Self {
        OSM {
            bbox: Bbox::new(),
            nodes: Vec::<Node>::new(),
            ways: Vec::<Way>::new(),
            relations: Vec::Relation::new(),
        }
    }

    pub fn parse(reader: EventReader) -> Result<Osm, ()> {
        let mut osm = OSM::new();


        for elem in reader {
            match elem {
                Ok(data) => {}
                _ => {}
            }
        }
    }
}