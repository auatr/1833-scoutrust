use firebase_rs::{Firebase};
use crate::config::data::{self, Match};

fn init_fb() -> Firebase {
    Firebase::new("https://scouting-1833.firebaseio.com").unwrap()
}

async fn set_match(send_match: data::Match, path: &str) {
    let fb = init_fb().at(path); 
    fb.set( &send_match).await;
}

async fn get_matches(path: &str) -> Match { // Gets all matches under path ie all matches in reefscape
    let fb = init_fb().at(path);
    fb.get::<Match>().await.expect("Failed to get matches")
}

async fn get_match(path: &str, id: &str) -> Match {
    let fb = init_fb().at(path).at(id);
    fb.get::<Match>().await.expect("Failed to get match with id {&path}")
}