use crate::connection::DbConn;
use crate::product::Product;
use crate::product::repository;
use crate::product::repository::NewProduct;

use std::env;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status; // https://api.rocket.rs/v0.4/rocket/response/status/struct.Created.html
use rocket_contrib::json::Json;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Product>>, Status> {
    repository::all(&connection)
        .map(|product| Json(product))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Product>, Status> {
    repository::get(id, &connection)
        .map(|product| Json(product))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<product>")]
pub fn post(product: Json<NewProduct>, connection: DbConn) -> Result<status::Created<Json<Product>>, Status> {
    repository::insert(product.into_inner(), &connection)
        .map(|product| product_created(product))
        .map_err(|error| error_status(error))
}

fn product_created(product: Product) -> status::Created<Json<Product>> {
    status::Created(
        format!("{host}:{port}/product/{id}", host = host(), port = port(), id = product.id).to_string(),
        Some(Json(product)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<product>")]
pub fn put(id: i32, product: Json<Product>, connection: DbConn) -> Result<Json<Product>, Status> {
    repository::update(id, product.into_inner(), &connection)
        .map(|product| Json(product))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match repository::get(id, &connection) {
        Ok(_) => repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
