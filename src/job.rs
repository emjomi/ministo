use serde::{Deserialize, Deserializer};

fn target_from_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let hex: String = Deserialize::deserialize(deserializer)?;
    Ok(u32::from_le_bytes(
        hex::decode(hex).unwrap().try_into().unwrap(),
    ))
}

#[derive(Debug, Clone, Deserialize)]
pub struct Job {
    #[serde(rename = "job_id")]
    pub id: String,
    #[serde(with = "hex")]
    pub blob: Vec<u8>,
    #[serde(rename = "seed_hash", with = "hex")]
    pub seed: Vec<u8>,
    #[serde(deserialize_with = "target_from_hex")]
    pub target: u32,
}

impl Job {
    pub fn difficulty(&self) -> u64 {
        u64::MAX / (u32::MAX / self.target) as u64
    }
}
