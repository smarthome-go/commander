use crate::Config;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct HasApiKey;

#[derive(Debug)]
pub enum HasApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HasApiKey {
    type Error = HasApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let config = req.rocket().state::<Config>().unwrap();

        fn is_valid(key: &str, token: &str) -> bool {
            key == token
        }

        match req.headers().get_one("token") {
            None => Outcome::Error((Status::Unauthorized, HasApiKeyError::Missing)),
            Some(key) if is_valid(key, &config.token) => Outcome::Success(HasApiKey),
            Some(_) => Outcome::Error((Status::Unauthorized, HasApiKeyError::Invalid)),
        }
    }
}
