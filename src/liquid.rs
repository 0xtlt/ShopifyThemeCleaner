use crate::{REGEX_INCLUDE, REGEX_RENDER, REGEX_SCHEMA, REGEX_SECTION};
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidSectionSchema {}

#[derive(Debug)]
pub struct LiquidFile {
    pub file_name: String,
    pub snippet_renders: Vec<String>,
    pub section_renders: Vec<String>,
    pub asset_renders: Vec<String>,
    pub schema: Option<LiquidSectionSchema>,
}

pub fn normalize_quotes(code: &str) -> String {
    code.replace('"', "\'")
}

fn extract_schema(content: &str) -> Option<String> {
    REGEX_SCHEMA
        .captures(content)
        .and_then(|caps| caps.get(1).map(|match_| match_.as_str().trim().to_string()))
}

pub fn parse_schema(content: &str) -> Option<LiquidSectionSchema> {
    extract_schema(content)
        .map(|schema| serde_json::from_str(&schema).unwrap_or(LiquidSectionSchema {}))
}

pub async fn parse_liquid_file(file_name: &str, code: &str) -> LiquidFile {
    let mut liquid_file = LiquidFile {
        file_name: file_name.to_string(),
        snippet_renders: vec![],
        section_renders: vec![],
        asset_renders: vec![],
        schema: parse_schema(code),
    };

    let normalized_code = normalize_quotes(code);

    for cap in REGEX_RENDER.captures_iter(&normalized_code) {
        if let Some(variable) = cap.name("variable") {
            let val = variable.as_str().to_string();
            // check if the value is not already in the list
            if !liquid_file.snippet_renders.contains(&val) {
                liquid_file.snippet_renders.push(val);
            }
        }
    }

    for cap in REGEX_INCLUDE.captures_iter(&normalized_code) {
        if let Some(variable) = cap.name("variable") {
            let val = variable.as_str().to_string();
            // check if the value is not already in the list
            if !liquid_file.asset_renders.contains(&val) {
                liquid_file.asset_renders.push(val);
            }
        }
    }

    for cap in REGEX_SECTION.captures_iter(&normalized_code) {
        if let Some(variable) = cap.name("variable") {
            let val = variable.as_str().to_string();
            // check if the value is not already in the list
            if !liquid_file.section_renders.contains(&val) {
                liquid_file.section_renders.push(val);
            }
        }
    }

    liquid_file
}

pub async fn read_and_parse_liquid_file(path: String) -> LiquidFile {
    let file_name = path.clone().split('/').last().unwrap().to_string();
    let content = fs::read_to_string(&path).await.unwrap();
    parse_liquid_file(&file_name, &content).await
}
