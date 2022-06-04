use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use std::path::Path;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hi {}, nice to meet you!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
        .mount("/public", FileServer::from("static/"))
}
