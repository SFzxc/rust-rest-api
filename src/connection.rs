use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::env;
use std::ops::Deref;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

fn database_url() -> String {
    // match env::var("DATABASE_URL") {
    //     Ok(val) => println!("{}: {:?}", "DATABASE_URL", val),
    //     Err(e) => println!("couldn't interpret {}: {}", "DATABASE_URL", e),
    // }
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

// https://docs.diesel.rs/1.4.x/diesel/r2d2/struct.PooledConnection.html
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

// https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
