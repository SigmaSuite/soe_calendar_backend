use rocket;
use crate::connection;
use crate::api;
use crate::cors;


pub fn create_routes(){
    rocket::ignite().attach(cors::CORS)
    .manage(connection::init_pool())
    .mount("/persons",
routes![
    api::handler::all_persons,
    api::handler::add_person,
    api::handler::get_person,
    api::handler::delete_person,
    api::handler::update_person

]).launch();
}