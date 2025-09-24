use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::{collections::HashMap, fs};
use ron::{Deserializer, Value};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Prematch {
    team_number: i32,
    match_number: i32 
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Auton {
    required: HashMap<String, Value>,
    pieces: HashMap<String, HashMap<String, i32>>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Teleop {
    pieces: HashMap<String, HashMap<String, i32>>,
    endgame: HashMap<String, HashMap<String, Value>>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Postmatch {
    checks: HashMap<String, bool> 
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Match {
    prematch: Prematch,
    auton: Auton,
    teleop: Teleop,
    postmatch: Postmatch,
}

impl Prematch {
    fn new() -> Self {
        Self { team_number: 0, match_number: 9999 }
    }
}

impl Auton {
    fn new() -> Self {
        Self { required: HashMap::new(), pieces: HashMap::new() }
    }
}

impl Teleop {
    fn new() -> Self {
        Self { pieces: HashMap::new(), endgame: HashMap::new() }
    }
}

impl Postmatch {
    fn new() -> Self{
        Self { checks: HashMap::new() }
    }
}

impl Match {
    fn new() -> Self {
        Self {
            prematch: Prematch::new(),
            auton: Auton::new(),
            teleop: Teleop::new(),
            postmatch: Postmatch::new(),
        }
    }
}

pub fn initialize_match() -> Match {
    let auton: Auton = ron::from_str(&fs::read_to_string("./auton_config.ron").expect("Failed to read auton_config")).unwrap();
    let teleop: Teleop = ron::from_str(&fs::read_to_string("./teleop_config.ron").expect("Failed to read teleop_config")).unwrap();
    let postmatch: Postmatch = ron::from_str(&fs::read_to_string("./postmatch_config.ron").expect("Failed to read postmatch")).unwrap();
    
    Match { prematch: Prematch::new(), auton: auton, teleop: teleop, postmatch: postmatch }
}

// Global shared data
pub static GLOBAL_DATA: Lazy<Mutex<Match>> = Lazy::new(|| Mutex::new(initialize_match()));
