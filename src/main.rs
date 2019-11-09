extern crate gotham;
extern crate hyper;
extern crate mime;

use gotham::router::Router;
use gotham::router::builder::*;

mod handlers;

mod game;

fn router() -> Router {
    build_simple_router(|route| {
        route.associate("/game", |route| {
            route.get().to(handlers::game::list);
            route.post();
            route.delete();
        });
    })
}

fn main() {
    let addr = "127.0.0.1:3000";
    println!("Server running on {}", addr);
    gotham::start(addr, router());
}