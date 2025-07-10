use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct SignupOutput {
  pub jwt: String
}

#[derive(Serialize, Deserialize)]
pub struct SigninOutput {
  pub jwt: String
}