use crate::store::Store;

impl Store {
    pub fn create_website(&self) -> String {
        String::from("Website created")
    }
    pub fn get_website(&self) -> String {
        String::from("Website 1")
    }
}
