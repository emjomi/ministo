#[derive(Debug)]
pub struct Share {
    pub nonce: Vec<u8>,
    pub hash: Vec<u8>,
    pub job_id: String,
}
