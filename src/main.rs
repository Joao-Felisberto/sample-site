#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::Cookies;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/owl")]
fn owl(cookies: Cookies) -> String {
    match cookies.get("flag").map(|c| c.value()) {
        Some(cookie) => {
            println!("{cookie}");
            String::from(cookie)
        }
        _ => {
            println!("FAILURE!");
            String::from("an error has occured")
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, owl]).launch();
}