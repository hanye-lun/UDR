use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::fmt::format;
use std::sync::Arc;
use serde_json::{to_string, Value};

#[tokio::main]
async fn main() {
    let ser = query_project_files().await;
    println!("{:#?}", ser)
}

async fn query_project_files(str: &str)
                             -> Result<String, Box<dyn std::error::Error>>
{
    let mut files: Vec<String> = Vec::new();
    let body: String = reqwest::get(format!("https://unpkg.com/{}/?meta", str)).await?
        .text().await?;
    let json: Arc<Value> = Arc::new((serde_json::from_str(&body)?) as Value);
    let mut temp = Arc::clone(&json);
    loop {
        match temp.borrow()["type"]?.as_str()? {
            "file" => {
                files.push(temp.borrow()["path"]?.as_str()?);
            }
            "directory" => {

            }
            _ => {}
        }
    }
    return Ok(json.to_string());
}