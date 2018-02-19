use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Welcome {
    #[serde(rename = "jsonrpc")]
    jsonrpc: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "result")]
    result: StatusResult,
}

#[derive(Serialize, Deserialize)]
pub struct StatusResult {
    #[serde(rename = "node_info")]
    pub node_info: NodeInfo,

    #[serde(rename = "pub_key")]
    pub pub_key: PubKey,

    #[serde(rename = "latest_block_hash")]
    pub latest_block_hash: String,

    #[serde(rename = "latest_app_hash")]
    pub latest_app_hash: String,

    #[serde(rename = "latest_block_height")]
    pub c: i64,

    #[serde(rename = "latest_block_time")]
    pub latest_block_time: String,

    #[serde(rename = "syncing")]
    pub syncing: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NodeInfo {
    #[serde(rename = "pub_key")]
    pub pub_key: String,

    #[serde(rename = "moniker")]
    pub moniker: String,

    #[serde(rename = "network")]
    pub network: String,

    #[serde(rename = "remote_addr")]
    pub remote_addr: String,

    #[serde(rename = "listen_addr")]
    pub listen_addr: String,

    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "other")]
    pub other: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PubKey {
    #[serde(rename = "type")]
    pub pub_key_type: String,

    #[serde(rename = "data")]
    pub data: String,
}

pub fn response_to_result(json: &str)-> Result<StatusResult, serde_json::Error>{
    let model: Welcome = serde_json::from_str(&json)?;
    Result::Ok(model.result)
}