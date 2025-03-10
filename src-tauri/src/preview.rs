use crate::session::{Session, SESSION};
use anyhow::{anyhow, Result};
use base64::Engine;
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;
impl Session {
    pub async fn get_preview(&self, id: u64) -> Result<(String, String)> {
        let url = format!("https://courses.zju.edu.cn/api/uploads/document/{id}/url?preview=true");
        let res = self.client.get(url).send().await?;
        let json = res.json::<Value>().await?;
        let url = json["url"]
            .as_str()
            .ok_or(anyhow!("预览请求返回无 url 字段"));
        let res = self.client.get(url?).send().await?;
        let headers = res.headers().clone();
        let content_type = headers
            .get(CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("application/octet-stream");
        let bytes = res.bytes().await?;
        let base_64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
        Ok((base_64, content_type.to_string()))
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_preview(id: u64) -> Result<(String, String), String> {
    SESSION
        .get_preview(id)
        .await
        .map_err(|e| e.to_string())
}
