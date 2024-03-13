use crate::job::Job;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct Response<R> {
    pub result: Option<R>,
    pub error: Option<Error>,
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct LoginResult {
    pub job: Job,
    pub id: String,
    pub status: String,
}

// Responses to subtit and keepalived requests differ only in the status value
#[derive(Debug, Deserialize)]
pub struct StatusResult {
    pub status: String,
}
