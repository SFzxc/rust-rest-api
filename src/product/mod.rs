#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::products;

pub mod handler;
pub mod router;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32
}
