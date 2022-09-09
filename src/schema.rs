use serde::Deserialize;

#[derive(FromForm, Deserialize)]
pub struct ExecRequest<'request> {
    pub command: &'request str,
}
