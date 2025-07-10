use std::sync::{Arc, Mutex};

use poem::{
    handler,
    web::{Data, Json},
};
use store::store::Store;

use crate::types::{
    req_input::{SigninInputs, SignupInputs},
    req_output::{SigninOutput, SignupOutput},
};

#[handler]
pub fn signup(
    Json(data): Json<SignupInputs>,
    Data(store): Data<&Arc<Mutex<Store>>>,
) -> Json<SignupOutput> {
    let mut locked_store = store.lock().unwrap();
    let user_id = locked_store.signup(data.username, data.password).unwrap();

    let response = SignupOutput { jwt: user_id };
    Json(response)
}

#[handler]
pub fn signin(
    Json(data): Json<SigninInputs>,
    Data(store): Data<&Arc<Mutex<Store>>>,
) -> Json<SigninOutput> {
    let mut locked_store = store.lock().unwrap();
    let user_id = locked_store.signin(data.username, data.password).unwrap();

    let response = SigninOutput { jwt: user_id };
    Json(response)
}
