use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request<P> {
    pub method: String,
    pub params: P,
    #[serde(skip_deserializing)]
    pub id: u32,
}

#[derive(Debug, Serialize)]
pub struct LoginParams {
    pub login: String,
    pub pass: String,
}

impl Request<LoginParams> {
    pub fn new(params: LoginParams) -> Self {
        Self {
            method: "login".into(),
            params,
            id: 1,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SubmitParams {
    pub id: String,
    pub job_id: String,
    #[serde(with = "hex")]
    pub nonce: Vec<u8>,
    #[serde(with = "hex")]
    pub result: Vec<u8>,
}

impl Request<SubmitParams> {
    pub fn new(params: SubmitParams) -> Self {
        Self {
            method: "submit".into(),
            params,
            id: 1,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct KeepAlivedParams {
    pub id: String,
}

impl Request<KeepAlivedParams> {
    pub fn new(params: KeepAlivedParams) -> Self {
        Self {
            method: "keepalived".into(),
            params,
            id: 1,
        }
    }
}
