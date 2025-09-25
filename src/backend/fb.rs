use firebase_rs::{Firebase};
use crate::config::data::{self, Data};

fn init_fb() -> Firebase {
    Firebase::new("https://scouting-1833.firebaseio.com").unwrap()
}

async fn set_Data(send_Data: data::Data, path: &str) {
    let fb = init_fb().at(path); 
    fb.set( &send_Data).await.expect("Failed to send Data");
}

async fn get_Database(path: &str) -> Data { // Gets all Dataes under path ie all Dataes in reefscape
    let fb = init_fb().at(path);
    fb.get::<Data>().await.expect("Failed to get Data")
}

async fn get_Match(path: &str, id: &str) -> Data {
    let fb = init_fb().at(path).at(id);
    fb.get::<Data>().await.expect("Failed to get Match with id {&path}")
}