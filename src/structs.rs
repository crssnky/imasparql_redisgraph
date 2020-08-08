extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use self::serde_derive::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Element {
  pub r#type: String,
  pub value: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Bindings {
  pub units: Element,
  pub idol: Element,
}
#[derive(Debug, Deserialize)]
pub struct Results {
  pub bindings: Vec<Bindings>,
}
#[derive(Debug, Deserialize)]
pub struct Head {
  pub vars: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct Response {
  pub head: Head,
  pub results: Results,
}
