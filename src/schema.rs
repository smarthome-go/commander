use serde::{Deserialize, Serialize};

#[derive(FromForm, Deserialize, Serialize)]
pub struct ExecRequest {
    pub command: String,
}
