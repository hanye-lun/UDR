use serde;
use serde::Serialize;
use serde::Deserialize;
use crate::op::file::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Directory {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "files")]
    pub files: Vec<DirectoryItem>,
}

impl Directory {

    pub fn subdirectories_all(&self) -> Option<Vec<&Directory>> {
        let mut dirs: Vec<&Directory> = Vec::new();
        dirs.push(self);
        while let Some(list_dir) = dirs.pop() {
            for item in list_dir.files.iter() {
                if let DirectoryItem::Directory(dir) = item {
                    dirs.push(dir);
                }
            }
        }
        return if dirs.is_empty() {
            None
        } else {
            Some(dirs)
        };
    }
    pub fn file_all(&self) -> Option<Vec<&File>>{
        let mut files: Vec<&File> = Vec::new();
        let mut dirs: Vec<&Directory> = Vec::new();
        dirs.push(self);
        while let Some(list_dir) = dirs.pop() {
            for item in list_dir.files.iter() {
                match item {
                    DirectoryItem::File(v)=>{
                        files.push(v);
                    },
                    DirectoryItem::Directory(v)=>{
                        dirs.push(v)
                    }
                }
            }
        }
        return if files.is_empty() {
            None
        } else {
            Some(files)
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DirectoryItem {
    #[serde(rename = "file")]
    File(File),
    #[serde(rename = "directory")]
    Directory(Directory),
}