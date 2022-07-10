use serde::{Serialize, Deserialize};

#[derive(FromForm, Deserialize, Serialize)]
pub struct ExecRequest {
    pub command: String,
}
