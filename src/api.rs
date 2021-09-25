use rocket::fairing::AdHoc;

#[get("/")]
pub fn index() -> &'static str {
    "HELLO FROM API"
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("API", |rocket| async {
        rocket.mount("/api", routes![index])
    })
}
