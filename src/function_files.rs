#![allow(unused_imports)]
#![allow(dead_code)]
use crate::functions::{address_url, call_reqwest};
use crate::json_struct::{FileInfo, Files, SimpleReturnJson, SimpleUploadReturnJson};
use reqwest::blocking::multipart;
use std::path::Path;
use std::time::Duration;

pub fn get_files() -> Result<Files, Box<dyn std::error::Error>> {
    let response = call_reqwest().get(address_url("/api/files", None)).send()?;
    let return_result: Files = response.json()?;
    Ok(return_result)
}

pub fn delete_file(file_name: &str) -> Result<SimpleReturnJson, Box<dyn std::error::Error>> {
    let response = call_reqwest()
        .delete(address_url("/api/files", Some(file_name)))
        .send()?;
    let result: SimpleReturnJson = response.json()?;
    Ok(result)
}

pub fn upload_file(
    file_path: &Path,
    file_name: String,
) -> Result<SimpleUploadReturnJson, Box<dyn std::error::Error>> {
    //https://docs.rs/reqwest/0.9.9/reqwest/multipart/index.html  -- 다중 업로드//안해도 가능;;;

    // Content-Disposition: form-data; name=files; filename=test.txt[\r][\n]
    let form = multipart::Form::new().file(file_name, file_path)?;
    let resp = call_reqwest()
        .post(address_url("/api/files", None))
        // .header("Content-Disposition", "form-data; name=files; filename=test 01.png\r\n")
        .timeout(Duration::from_secs(1000))
        .multipart(form)
        .send()?;
    println!("{:?}", address_url("/api/files", None));
    println!("{:?}", resp);
    let result: SimpleUploadReturnJson = resp.json()?;
    Ok(result)
}

pub fn get_files_info(files: Vec<String>) -> Vec<FileInfo> {
    let mut t = Vec::new();
    for n in files {
        t.push(FileInfo::new(n))
    }
    t
}
#[test]
fn test_files() {}
