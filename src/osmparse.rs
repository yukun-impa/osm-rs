use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use xml::reader::{EventReader, XmlEvent};
use xml::name::{OwnedName, OwnedAttribute};
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

    pub fn parse(reader: EventReader) -> Result<Self, ()> {
        let mut osm = OSM::new();
        let mut depth = OsmElement::Node;
        let mut node = Node::new();
        let mut way = Way::new();
        let mut relation = Relation::new();


        for elem in reader {
            match elem {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) if &name.local_name == "bounds" => {
                    osm.bounds_handler(name, atrributes);
                }


                Ok(XmlEvent::StartElement {
                    name, atrributes, ..
                }) if &name.local_name == "node" => {
                    node.read(name, attributes);
                }

                Ok(XmlEvent::EndElement {
                    name, atrributes, ..
                }) if &name.local_name == "node" => {
                    osm.nodes_handler(&node);

                }

                Ok(XmlEvent::StartElement {
                    name, atrributes, ..
                }) if &name.local_name == "way" => {

                }

                Ok(XmlEvent::EndElement {
                    name, atrributes, ..
                }) if &name.local_name == "way" => {

                }
                Ok(XmlEvent::StartElement {
                    name, atrributes, ..
                }) if &name.local_name == "relation" => {
                }
                Ok(XmlEvent::EndElement {
                    name, atrributes, ..
                }) if &name.local_name == "relation" => {
                }
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) if &name.local_name == "tag" => {
                    match depth {
                        Node => {},
                        Way => {},
                        Relation => {},
                    }

                }
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) if &name.local_name == "member" => {

                }
                _ => {}
            }
        }
        Err(())
    }

    fn bounds_handler(&mut self, name: OwnedName, atrributes: Vec<OwnedAttribute>) {

        for elem in atrributes {
            if &e.name.local_name == "minlon" {
                self.bbox.left = e.value.parse::<f64>().unwrap();
            }

            if &e.name.local_name == "minlat" {
                self.bbox.bottom = e.value.parse::<f64>().unwrap();
            }

            if &e.name.local_name == "maxlon" {
                self.bbox.right = e.value.parse::<f64>().unwrap();
            }

            if &e.name.local_name == "maxlat" {
                self.bbox.top = e.value.parse::<f64>().unwrap();
            }
        }
    }

    fn nodes_handler(&mut self, node: &Node) {
        self.nodes.push(node.clone());
    }

    fn ways_handler(&mut self, way: &Way) {

    }

    fn relations_handler(&mut self, name: OwnedName, atrributes: Vec<OwnedAttribute>, depth: OsmElement) {

    }
}