use std::env;
use rocket_contrib::json::Json;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;

use crate::connection::DbConn;
use crate::api::models::*;
use crate::api::repository;


#[get("/")]
pub fn all_persons(connection: DbConn) -> Result<Json<Vec<Person>>, Status> {
    println!("dupa");
    repository::get_all_persons(&connection)
        .map(|person| Json(person))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_person(id: i32, connection: DbConn) -> Result<Json<Person>, Status> {
    repository::get_person(id, &connection)
        .map(|person| Json(person))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_person(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    repository::delete_person(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

#[post("/", format ="application/json", data = "<new_person>")]
pub fn add_person(new_person: Json<NewPerson>, connection: DbConn) ->  Result<Json<Person>, Status> {
    repository::add_person(new_person.into_inner(), &connection)
        .map(|new_person| Json(new_person))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<person>")]
pub fn update_person(id: i32, person: Json<Person>, connection: DbConn) -> Result<Json<Person>, Status> {
    repository::update_person(id, person.into_inner(), &connection)
        .map(|person| Json(person))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
