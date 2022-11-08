use crate::reader::osmelements::NetworkType;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Filter {
    pub network_type: NetworkType,
    pub tags_values_exclude: HashMap<String, Vec<String>>,
}

impl NetworkType {
    pub fn get_filter(&self) -> Filter {
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
            NetworkType::Drive => {
                tags_values_exclude.insert("area".to_string(), vec!["yes".to_string()]);
                tags_values_exclude.insert(
                    "highway".to_string(),
                    vec![
                        "abandoned".to_string(),
                        "bridleway".to_string(),
                        "bus_guideway".to_string(),
                        "construction".to_string(),
                        "corridor".to_string(),
                        "cycleway".to_string(),
                        "elevator".to_string(),
                        "escalator".to_string(),
                        "footway".to_string(),
                        "path".to_string(),
                        "pedestrian".to_string(),
                        "planned".to_string(),
                        "platform".to_string(),
                        "proposed".to_string(),
                        "raceway".to_string(),
                        "service".to_string(),
                        "steps".to_string(),
                        "track".to_string(),
                    ],
                );
                tags_values_exclude.insert("motor_vehicle".to_string(), vec!["no".to_string()]);
                tags_values_exclude.insert("motor_car".to_string(), vec!["no".to_string()]);
                tags_values_exclude.insert(
                    "service".to_string(),
                    vec![
                        "private".to_string(),
                        "emergency_access".to_string(),
                        "parking".to_string(),
                        "parking_aisle".to_string(),
                    ],
                );
            }
            NetworkType::All => {
                tags_values_exclude.insert("area".to_string(), vec!["yes".to_string()]);
                tags_values_exclude.insert(
                    "highway".to_string(),
                    vec![
                        "abandoned".to_string(),
                        "construction".to_string(),
                        "planned".to_string(),
                        "platform".to_string(),
                        "proposed".to_string(),
                        "raceway".to_string(),
                    ],
                );
                tags_values_exclude.insert("service".to_string(), vec!["private".to_string()]);
            }

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
