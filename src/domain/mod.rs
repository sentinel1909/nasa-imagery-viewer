// domain app data

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct NASAData {
    pub date: String,
    pub title: String,
    pub explanation: String,
    pub hdurl: String,
}