use crate::reader::osmelements::{Bbox, NetworkType, Node, OsmElement, Relation, Tag, Way};
use std::fs::File;
use std::io::BufReader;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::{EventReader, XmlEvent};

pub struct OSM {
    pub bbox: Bbox,
    pub nodes: Vec<Node>,
    pub ways: Vec<Way>,
    pub relations: Vec<Relation>,
}

impl OSM {
    pub fn new() -> Self {
        OSM {
            bbox: Bbox::new(),
            nodes: Vec::<Node>::new(),
            ways: Vec::<Way>::new(),
            relations: Vec::<Relation>::new(),
        }
    }

    pub fn parse(path: &str) -> Result<Self, ()> {
        let fileread = File::open(path).unwrap();
        let fileread = BufReader::new(fileread);
        let reader = EventReader::new(fileread);

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
                    osm.bounds_handler(name, attributes);
                }

                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) if &name.local_name == "node" => {
                    depth = OsmElement::Node;
                    node = Node::with_attributes(attributes);
                }

                Ok(XmlEvent::EndElement { name, .. }) if &name.local_name == "node" => {
                    osm.nodes_handler(&node);
                }

                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) if &name.local_name == "way" => {
                    depth = OsmElement::Way;
                    way = Way::with_attributes(attributes);
                }

                Ok(XmlEvent::EndElement { name, .. }) if &name.local_name == "way" => {
                    osm.ways_handler(&way);
                }

                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) if &name.local_name == "relation" => {
                    depth = OsmElement::Relation;
                    relation = Relation::with_attributes(attributes);
                }

                Ok(XmlEvent::EndElement { name, .. }) if &name.local_name == "relation" => {
                    osm.relations_handler(&relation);
                }

                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) if &name.local_name == "tag" => match &depth {
                    &OsmElement::Node => {
                        node.add_tag(attributes);
                    }
                    &OsmElement::Way => {
                        way.add_tag(attributes);
                    }
                    &OsmElement::Relation => relation.add_tag(attributes),
                },

                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) if &name.local_name == "member" => {
                    relation.add_member(attributes);
                }
                Err(e) => panic!("Error: {}", e),
                _ => {}
            }
        }
        Ok(osm)
    }

    fn bounds_handler(&mut self, name: OwnedName, atrributes: Vec<OwnedAttribute>) {
        for elem in atrributes {
            if &elem.name.local_name == "minlon" {
                self.bbox.left = elem.value.parse::<f64>().unwrap();
            }

            if &elem.name.local_name == "minlat" {
                self.bbox.bottom = elem.value.parse::<f64>().unwrap();
            }

            if &elem.name.local_name == "maxlon" {
                self.bbox.right = elem.value.parse::<f64>().unwrap();
            }

            if &elem.name.local_name == "maxlat" {
                self.bbox.top = elem.value.parse::<f64>().unwrap();
            }
        }
    }

    fn nodes_handler(&mut self, node: &Node) {
        self.nodes.push(node.clone());
    }

    fn ways_handler(&mut self, way: &Way) {
        self.ways.push(way.clone());
    }

    fn relations_handler(&mut self, relation: &Relation) {
        self.relations.push(relation.clone());
    }

    fn filter_ways(&mut self, networktype: &NetworkType) {
        let mut filter = networktype.get_filter();
        self.ways = self
            .ways
            .iter()
            .filter(|&way| way.tag_valid(&mut filter))
            .cloned()
            .collect();
    }
}
