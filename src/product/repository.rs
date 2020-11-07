#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::products;
use crate::product::Product;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Product>> {
    products::table.load::<Product>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Product> {
    products::table.find(id).get_result::<Product>(connection)
}

pub fn insert(product: NewProduct, connection: &PgConnection) -> QueryResult<Product> {
    diesel::insert_into(products::table)
        .values(&NewProduct::from_product(product))
        .get_result(connection) // https://docs.diesel.rs/diesel/query_dsl/trait.RunQueryDsl.html#method.get_result
}

pub fn update(id: i32, product: Product, connection: &PgConnection) -> QueryResult<Product> {
    diesel::update(products::table.find(id))
        .set(&product)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(products::table.find(id))
        .execute(connection)
}

#[derive(Insertable, Deserialize)]
#[table_name = "products"]
pub struct NewProduct {
    name: String,
    price: i32
}

impl NewProduct {

    fn from_product(product: NewProduct) -> NewProduct {
        NewProduct {
            name: product.name,
            price: product.price
        }
    }
}
