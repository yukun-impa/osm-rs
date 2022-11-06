use std::collections::HashMap;
use std::collections::HashSet;
use crate::elements::{Bbox, Node, Way, NetworkType};
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use xml::reader::EventReader;
use xml::reader::XmlEvent;

pub struct Filter {
    network_type: NetworkType,
    tags_values_exclude: HashMap<String, Vec<String>>,
}

impl NetworkType {
    pub fn get_filter(&self) -> Filter {
        let mut tags_contain = HashSet::<String>::new();
        let mut tags_values_exclude = HashMap::<String, Vec<String>>::new();
        tags_values_exclude.insert("access".to_string(), vec!["private".to_string()]);
        match *self {
            NetworkType::Walk => {
                tags_values_exclude.insert("area".to_string(), vec!["yes".to_string()]);
                tags_values_exclude.insert(
                    "highway".to_string(),
                    vec![
                        "abandoned".to_string(),
                        "bridleway".to_string(),
                        "bus_guideway".to_string(),
                        "construction".to_string(),
                        "cycleway".to_string(),
                        "motor".to_string(),
                        "planned".to_string(),
                        "platform".to_string(),
                        "proposed".to_string(),
                        "raceway".to_string(),
                    ],
                );
                tags_values_exclude.insert("foot".to_string(), vec!["no".to_string()]);
                tags_values_exclude.insert("service".to_string(), vec!["private".to_string()]);
            }
            NetworkType::Drive => {}

            _ => {}
        }
        Filter {
            network_type: *self,
            tags_values_exclude,
        }
    }
}

impl FromStr for NetworkType {
    type Err = ();

    fn from_str(input: &str) -> Result<NetworkType, Self::Err> {
        match input {
            "walk" => Ok(NetworkType::Walk),
            "bike" => Ok(NetworkType::Bike),
            "drive" => Ok(NetworkType::Drive),
            "all" => Ok(NetworkType::All),
            "all_private" => Ok(NetworkType::AllPrivate),
            "drive_service" => Ok(NetworkType::DriveService),
            _ => Err(()),
        }
    }
}
