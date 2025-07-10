use poem::{
    handler,
    // web::{Data, Json, Path},
};

#[handler]
pub fn create_website() -> String {
    String::from("hello")
}
