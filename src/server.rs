use rocket::{routes, Rocket};

pub fn server() -> Rocket{
    rocket::ignite().mount("/api/v1.0/", routes![crate::handlers::root, crate::handlers::get_projects])
}