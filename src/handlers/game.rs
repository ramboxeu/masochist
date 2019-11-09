extern crate mime;
extern crate serde_json;

use gotham::state::State;
use crate::game::{ Game };
use gotham::helpers::http::response::create_response;
use hyper::{Response, Body, StatusCode};

pub fn list(state: State) -> (State, Response<Body>) {
    let games = [
        Game::new(String::from("Portal"), String::from("400")),
        Game::new(String::from("Portal 2"), String::from("620")),
        Game::new(String::from("Factorio"), String::from("427520"))
    ];

    let res = create_response(
        &state,
        StatusCode::OK,
        mime::APPLICATION_JSON,
        serde_json::to_vec(&games).expect("serialized games")
    );

    (state, res)
}