use crate::config::Config;
use crate::guards::HasApiKey;
use crate::schema::ExecRequest;
use async_process::Command;
use rocket::{serde::json::Json, State};

use serde::Serialize;
use std::io;

#[get("/")]
pub fn index_handler() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
pub struct CommandResponse {
    std_out: String,
    std_err: String,
    success: bool,
}

#[post("/exec", format = "json", data = "<request>")]
pub async fn exec_handler(
    state: &State<Config>,
    _key: HasApiKey,
    request: Json<ExecRequest<'_>>,
) -> Result<Json<CommandResponse>, io::Error> {
    let out = Command::new(&state.shell)
        .arg("-c")
        .arg(&request.command)
        .output()
        .await?;

    Ok(Json(CommandResponse {
        std_out: String::from_utf8(out.stdout).unwrap_or_else(|_| String::from("invalid utf-8")),
        std_err: String::from_utf8(out.stderr).unwrap_or_else(|_| String::from("invalid utf-8")),
        success: out.status.success(),
    }))
}
