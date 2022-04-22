mod op;

use crate::op::directory::DirectoryItem;
use crate::op::file::File;
use reqwest::Client;
use std::borrow::Borrow;
use std::hash::Hasher;
use std::sync::{Arc, Mutex};
use tokio::fs::{create_dir_all, OpenOptions};
use tokio::net::windows::named_pipe::PipeEnd::Client;

#[tokio::main]
async fn main() {
    let ser = query_project("react@16.7.0").await;
    match ser {
        Ok(rs) => {
            if let DirectoryItem::Directory(s) = rs {
                let files = s.file_all().expect("未找到文件");
            }
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    }
}

async fn query_project(project_name: &str) -> Result<DirectoryItem, Box<dyn std::error::Error>> {
    let body: String = reqwest::get(format!("https://unpkg.com/{}/?meta", project_name))
        .await?
        .text()
        .await?;
    let project: DirectoryItem = serde_json::from_str(&body)?;
    return Ok(project);
}

async fn download_file(project_name: &str, files: Vec<&File>) {
    let files_iter = Arc::new(Mutex::new(files.iter()));
    for i in 0..5 {
        async {
            let client: Client = Client::new();
            let iter_arc = Arc::clone(&files_iter);
            loop {
                let mut iter = iter_arc.lock().unwrap();
                if let Some(&&file) = iter.next() {
                    let req = client
                        .get(format!("https://unpkg.com/{}/{}", project_name, &file.path))
                        .header("contentType", &file.content_type)
                        .send()
                        .await?;
                } else {
                    break;
                }
            }
        }
    }
}

async fn file_write(data: String) {
    let mut file_operation = OpenOptions::new();
    file_operation.create(true).write(true);
    //新建文件夹
    create_dir_all(format!("{}/{}", this_clone.2, file_i.0)).unwrap();
    //新建文件并写入
    file_operation.open(format!("{}/{}", this_clone.2, url)).w;
    println!("{}{} -> {}/{}", this_clone.1, url, this_clone.2, url);
}
