#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::Rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

fn assemble_rocket() -> Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    assemble_rocket().launch();
}

#[cfg(test)]
mod test {
    use super::assemble_rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn index_hello_world() {
        let client = Client::new(assemble_rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.body_string(), Some("Hello World!".into()));
    }

    #[test]
    fn unknown_route_error_404() {
        let client = Client::new(assemble_rocket()).expect("valid rocket instance");
        let mut response = client.get("/notexsitingroute").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}