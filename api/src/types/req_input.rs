use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignupInputs {
  pub username: String,
  pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct SigninInputs {
  pub username: String,
  pub password: String
}