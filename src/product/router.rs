use crate::product;
use crate::rocket;
use crate::connection;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/products",
            routes![product::handler::all,
            product::handler::get,
            product::handler::post,
            product::handler::put,
            product::handler::delete],
        ).launch();
}
