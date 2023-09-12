use serde::Deserialize;

#[derive(FromForm, Deserialize)]
pub struct ExecRequest {
    pub command: String,
    pub args: Vec<String>,
}
