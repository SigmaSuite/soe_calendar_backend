#![allow(proc_macro_derive_resolution_fallback)] // ????? XD

use diesel;
use diesel::prelude::*;

use crate::api::models::*;
use crate::schema::*;


pub fn get_person(id: i32, connection: &PgConnection) -> QueryResult<Person>{
    persons::table.find(id).get_result::<Person>(connection)
}

pub fn get_all_persons(connection: &PgConnection) -> QueryResult<Vec<Person>>{
    persons::table.load::<Person>(&*connection)
}

pub fn add_person(new_person: NewPerson, connection: &PgConnection) -> QueryResult<Person>{
    diesel::insert_into(persons::table)
    .values(&new_person)
    .get_result(connection)
}

pub fn update_person(id: i32, person: Person, connection: &PgConnection) -> QueryResult<Person>{
    diesel::update(persons::table.find(id))
    .set(&person)
    .get_result(connection)
}

pub fn delete_person(id: i32, connection: &PgConnection) -> QueryResult<usize>{
    diesel::delete(persons::table.find(id))
    .execute(connection)
}


