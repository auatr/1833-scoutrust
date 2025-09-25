use dioxus::signals::{Global, GlobalSignal, Signal};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{Number, Value};
use std::sync::Mutex;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigItem {
    title: String,
    key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigEntry {
    title: String,
    #[serde(rename = "type")]
    entry_type: String,
    items: Vec<ConfigItem>,
}

#[derive(Debug, Clone)]
pub struct Data {
    prematch: HashMap<String, HashMap<String, Value>>,
    auton: HashMap<String, HashMap<String, Value>>,
    teleop: HashMap<String, HashMap<String, Value>>,
    postmatch: HashMap<String, HashMap<String, Value>>,
}

impl Data {
    fn new() -> Self {
        Self {
            prematch: HashMap::new(),
            auton: HashMap::new(),
            teleop: HashMap::new(),
            postmatch: HashMap::new(),
        }
    }

    fn initialize(&mut self, config: &[ConfigEntry], phase: &str) {
        let phase_map = self.get_phase_data_mut(phase);
        for entry in config {
            let mut entry_map = HashMap::new();
            for item in &entry.items {
                let value = match entry.entry_type.to_lowercase().as_str() {
                    "number" | "integer" => Value::Number(Number::from(0)),
                    "boolean" => Value::Bool(false),
                    "string" | "input" => Value::String(String::new()),
                    _ => Value::Null,
                };

                entry_map.insert(item.key.clone(), value);
            }
            phase_map.insert(entry.title.clone(), entry_map);
        }
    }

    pub fn add(&mut self, phase: &str, title: &str, item_key: &str, value: Value) {
        let phase_map = self.get_phase_data_mut(phase);
        if let Some(entry_map) = phase_map.get_mut(title) {
            entry_map.insert(item_key.to_string(), value);
        } else {
            panic!("Title '{}' not found in phase '{}'", title, phase);
        }
    }

    pub fn remove(&mut self, phase: &str, title: &str, item: &str) {
        let phase_map = self.get_phase_data_mut(phase);
        if let Some(entry_map) = phase_map.get_mut(title) {
            entry_map.remove(item);
        } else {
            panic!("Title '{}' not found in phase '{}'", title, phase);
        }
    }

    pub fn get(&self, phase: &str, title: &str, item: &str) -> Option<&Value> {
        let phase_map = self.get_phase_data(phase)?;
        phase_map
            .get(title)
            .and_then(|entry_map| entry_map.get(item))
    }

    pub fn reset(&mut self) {
        for phase_map in [
            &mut self.prematch,
            &mut self.auton,
            &mut self.teleop,
            &mut self.postmatch,
        ] {
            for entry_map in phase_map.values_mut() {
                for value in entry_map.values_mut() {
                    *value = match value {
                        Value::Number(_) => Value::Number(Number::from(0)),
                        Value::Bool(_) => Value::Bool(false),
                        Value::String(_) => Value::String(String::new()),
                        _ => Value::Null,
                    };
                }
            }
        }
    }

    pub fn print_phase(&self, phase: &str) {
        if let Some(phase_map) = self.get_phase_data(phase) {
            println!("Data for phase '{}':", phase);
            for (title, entry_map) in phase_map {
                println!("  Title: {}", title);
                for (key, value) in entry_map {
                    println!("    {}: {}", key, value);
                }
            }
        } else {
            println!("Phase '{}' not found.", phase);
        }
    }

    fn get_phase_data(&self, phase: &str) -> Option<&HashMap<String, HashMap<String, Value>>> {
        match phase {
            "prematch" => Some(&self.prematch),
            "auton" => Some(&self.auton),
            "teleop" => Some(&self.teleop),
            "postmatch" => Some(&self.postmatch),
            _ => None,
        }
    }

    fn get_phase_data_mut(&mut self, phase: &str) -> &mut HashMap<String, HashMap<String, Value>> {
        match phase {
            "prematch" => &mut self.prematch,
            "auton" => &mut self.auton,
            "teleop" => &mut self.teleop,
            "postmatch" => &mut self.postmatch,
            _ => panic!("Unknown phase '{}'", phase),
        }
    }
}

pub fn load_config<T: DeserializeOwned>(file_name: &str) -> T {
    let raw = match file_name {
        "prematchConfig.json" => include_str!("../../assets/config/prematchConfig.json"),
        "autonConfig.json" => include_str!("../../assets/config/autonConfig.json"),
        "teleopConfig.json" => include_str!("../../assets/config/teleopConfig.json"),
        "postmatchConfig.json" => include_str!("../../assets/config/postmatchConfig.json"),
        _ => panic!("Unknown config file: {}", file_name),
    };

    serde_json::from_str(raw).unwrap_or_else(|_| panic!("Failed to parse JSON in {}", file_name))
}

pub fn initialize_data() -> Data {
    let mut data = Data::new();

    let prematch_config = load_config::<Vec<ConfigEntry>>("prematchConfig.json");
    let auton_config = load_config::<Vec<ConfigEntry>>("autonConfig.json");
    let teleop_config = load_config::<Vec<ConfigEntry>>("teleopConfig.json");
    let postmatch_config = load_config::<Vec<ConfigEntry>>("postmatchConfig.json");

    data.initialize(&prematch_config, "prematch");
    data.initialize(&auton_config, "auton");
    data.initialize(&teleop_config, "teleop");
    data.initialize(&postmatch_config, "postmatch");

    data
}

// Global shared data
pub static GLOBAL_DATA: GlobalSignal<Data> = Signal::global(|| initialize_data());
