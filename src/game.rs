use serde::{Serialize};

#[derive(Serialize)]
pub struct Game {
    title: String,
    steam_id: String,
}

impl Game {
    pub fn new(title: String, steam_id: String) -> Game {
        Game {
            title,
            steam_id
        }
    }
}