use poem::{EndpointExt, Route, Server, listener::TcpListener, post};
use std::sync::{Arc, Mutex};
use store::store::Store;

use crate::routes::user::{signin, signup};

pub mod routes;
pub mod types;

#[tokio::main()]
async fn main() -> Result<(), std::io::Error> {
    let s = Store::new().unwrap_or_else(|_| panic!("Error connecting to the database!"));
    let store = Arc::new(Mutex::new(s));

    let app = Route::new()
        .at("/api/user/signup", post(signup))
        .at("/api/user/signin", post(signin))
        .data(store);

    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .name("zero-downtime")
        .run(app)
        .await
}
