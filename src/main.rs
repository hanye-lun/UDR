mod op;

use crate::op::directory::DirectoryItem;
use crate::op::file::File;
use reqwest::Client;
use std::borrow::Borrow;
// use std::fs::{create_dir_all, OpenOptions};
use std::hash::Hasher;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tokio::fs::{create_dir_all, OpenOptions};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let project_name = "react@16.7.0";
    let project_rs = query_project("react@16.7.0").await;
    match project_rs {
        Ok(project) => {
            if let DirectoryItem::Directory(dir) = project {
                let files = dir.file_all().expect("未找到文件");
                download_file(project_name, "D:\\", files).await.unwrap();
            }
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    }
}

async fn query_project(project_name: &str) -> Result<DirectoryItem, Box<dyn std::error::Error>> {
    let body: String = reqwest::get(format!("https://unpkg.com/{}/?meta", project_name)).await?
        .text().await?;
    let project: DirectoryItem = serde_json::from_str(&body)?;
    return Ok(project);
}

async fn download_file(project_name: &str, file_path: &str, files: Vec<&File>) -> Result<(), Box<dyn std::error::Error>> {
    let files_iter = Arc::new(Mutex::new(files.iter()));
    let client: Client = Client::new();
    let iter_arc = Arc::clone(&files_iter);
    while let Some(&file) = iter_arc.lock().unwrap().next() {
        let req = client
            .get(format!("https://unpkg.com/{}/{}", project_name, &file.path))
            .header("contentType", &file.content_type)
            .send().await?;
        let mut _file_path = format!("{}\\{}\\{}", file_path, project_name, file.path);
        _file_path = _file_path.replace("/", "\\\\");
        println!("下载 : {}", _file_path);
        let file_path: (&str, &str) = _file_path.rsplit_once("\\").unwrap();
        file_write(
            file_path.0,
            file_path.1,
            &req.bytes().await?.to_vec(),
        ).await?;
    }
    return Ok(());
}

async fn file_write(path: &str, file_name: &str, data: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_operation = OpenOptions::new();
    file_operation.create(true)
        .append(false)
        .write(true);
    //新建文件夹
    let create = create_dir_all(path).await;
    match create {
        Ok(..) => {
            let _path = path.to_string() + file_name;
            let mut file = file_operation.open(_path).await?;
            file.write(data).await?;
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }
    return Ok(());
}
