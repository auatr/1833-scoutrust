use dioxus::signals::{GlobalSignal, Signal};
use indexmap::IndexMap;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{Number, Value};
use std::sync::LazyLock;

#[derive(Clone, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigItem {
    pub title: String,
    pub key: String,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigEntry {
    pub title: String,
    pub items: Vec<ConfigItem>,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub prematch: IndexMap<String, IndexMap<String, Value>>,
    pub auton: IndexMap<String, IndexMap<String, Value>>,
    pub teleop: IndexMap<String, IndexMap<String, Value>>,
    pub postmatch: IndexMap<String, IndexMap<String, Value>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            prematch: IndexMap::new(),
            auton: IndexMap::new(),
            teleop: IndexMap::new(),
            postmatch: IndexMap::new(),
        }
    }

    pub fn initialize(&mut self, config: &[ConfigEntry], phase: &str) {
        let phase_map = self.get_phase_data_mut(phase);

        for entry in config {
            let mut entry_map = IndexMap::new();
            for item in &entry.items {
                let value = match item.item_type.to_lowercase().as_str() {
                    "number" => Value::Number(Number::from(0)),
                    "boolean" => Value::Bool(false),
                    "string" | "text-input" | "int-input" => Value::String(String::new()),
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

    pub fn get_mut(&mut self, phase: &str, title: &str, item: &str) -> Option<&mut Value> {
        let phase_map = self.get_phase_data_mut(phase);
        phase_map
            .get_mut(title)
            .and_then(|entry_map| entry_map.get_mut(item))
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

    pub fn convert_to_json(&self, phase: &str) -> Option<String> {
        let phase_map = self.get_phase_data(phase)?;
        serde_json::to_string_pretty(phase_map).ok()
    }

    pub fn convert_all_to_json(&self) -> String {
        let all_data = IndexMap::from([
            ("prematch".to_string(), &self.prematch),
            ("auton".to_string(), &self.auton),
            ("teleop".to_string(), &self.teleop),
            ("postmatch".to_string(), &self.postmatch),
        ]);

        serde_json::to_string_pretty(&all_data).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn get_phase_data(
        &self,
        phase: &str,
    ) -> Option<&IndexMap<String, IndexMap<String, Value>>> {
        match phase {
            "prematch" => Some(&self.prematch),
            "auton" => Some(&self.auton),
            "teleop" => Some(&self.teleop),
            "postmatch" => Some(&self.postmatch),
            _ => None,
        }
    }

    pub fn get_phase_data_mut(
        &mut self,
        phase: &str,
    ) -> &mut IndexMap<String, IndexMap<String, Value>> {
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

    data.initialize(&PREMATCH_CONFIG.as_ref(), "prematch");
    data.initialize(&AUTON_CONFIG.as_ref(), "auton");
    data.initialize(&TELEOP_CONFIG.as_ref(), "teleop");
    data.initialize(&POSTMATCH_CONFIG.as_ref(), "postmatch");

    data
}

// Global shared data - now public
pub static PREMATCH_CONFIG: LazyLock<Vec<ConfigEntry>> =
    LazyLock::new(|| load_config::<Vec<ConfigEntry>>("prematchConfig.json"));
pub static AUTON_CONFIG: LazyLock<Vec<ConfigEntry>> =
    LazyLock::new(|| load_config::<Vec<ConfigEntry>>("autonConfig.json"));
pub static TELEOP_CONFIG: LazyLock<Vec<ConfigEntry>> =
    LazyLock::new(|| load_config::<Vec<ConfigEntry>>("teleopConfig.json"));
pub static POSTMATCH_CONFIG: LazyLock<Vec<ConfigEntry>> =
    LazyLock::new(|| load_config::<Vec<ConfigEntry>>("postmatchConfig.json"));

pub static GLOBAL_DATA: GlobalSignal<Data> = Signal::global(|| initialize_data());
