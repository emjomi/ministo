use crate::job::Job;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Response<R> {
    pub result: Option<R>,
    pub error: Option<Error>,
    pub id: u32,
}

#[allow(dead_code)]
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
