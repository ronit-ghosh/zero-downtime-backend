use poem::{
    Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Json, Path},
};
// use store::store::Store;
use crate::types::{req_input::CreateWebsiteInputs, req_output::CreateWebsiteOutputs};

pub mod types;

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInputs>) -> Json<CreateWebsiteOutputs> {
    let url = data.url;
    // let s = Store::default().unwrap();
    println!("{}", url);
    let response = CreateWebsiteOutputs {
        id: String::from("1"),
    };
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/get/website/:name", get(get_website))
        .at("/create/website", post(create_website));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
