use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateJson {
    pub sections: HashMap<String, TemplateSection>,
    pub order: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

pub async fn read_json_templates(folder_path: &Path) -> Vec<TemplateJson> {
    let mut json_files: Vec<TemplateJson> = Vec::new();
    let mut dir = fs::read_dir(folder_path).await.unwrap();

    while let Ok(Some(entry)) = dir.next_entry().await {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("json") {
            let content = fs::read_to_string(&path).await.unwrap();
            let template_json: TemplateJson = serde_json::from_str(&content).unwrap();
            json_files.push(template_json);
        }
    }

    json_files
}
