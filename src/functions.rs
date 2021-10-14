#![allow(unused_imports)]
use crate::function_files::get_files;
use crate::function_playlist::get_playlist;
use crate::function_status::get_status;
use dotenv::dotenv;
use pwhash::bcrypt;
use tera::{Context, Tera};

pub(crate) fn call_reqwest() -> reqwest::blocking::Client {
    reqwest::blocking::Client::new()
}

fn api_url() -> String {
    dotenv().ok();
    let string = std::env::var("SERVER_URL").expect("can not find api_address");
    string
}

pub fn address_url(addr: &str, var: Option<&str>) -> String {
    let mut url = api_url();
    url.push_str(addr);
    match var {
        None => {}
        Some(v) => {
            let add_var = format!("/{}", v);
            url.push_str(add_var.as_str());
        }
    }
    url
}

pub fn is_video(file_name: &str) -> bool {
    let is_video = file_name.contains("mp4")
        || file_name.contains("avi")
        || file_name.contains("mov")
        || file_name.contains("mkv")
        || file_name.contains("wmv")
        || file_name.contains("flv")
        || file_name.contains("ts")
        || file_name.contains("tp")
        || file_name.contains("3gp")
        || file_name.contains("mpg")
        || file_name.contains("mpeg")
        || file_name.contains("mpe")
        || file_name.contains("dat")
        || file_name.contains("rm");
    is_video
}

pub fn set_ticker_message(
    mut first: String,
    mut second: String,
    mut third: String,
    mut color: String,
) -> String {
    color.push_str("#0,0#");
    first.push_str("_");
    second.push_str("_");
    third.push_str("_");
    color.push_str(&first);
    color.push_str(&second);
    color.push_str(&third);
    color
}

pub fn get_verify(id: String, pw: String) -> Result<bool, Box<dyn std::error::Error>> {
    dotenv().ok();
    Ok(true)
    // let id_env = std::env::var("ID")?;
    // let pw_env = std::env::var("PW")?;
    // return if id_env.eq("test") {
    //     let verify = bcrypt::verify(pw, pw_env.as_str());
    //     Ok(verify)
    // } else {
    //     Ok(false)
    // };
}

pub fn get_context_for_main() -> Result<Context, Box<dyn std::error::Error>> {
    let mut context = Context::new();

    let files = get_files()?;
    let status = get_status()?;
    let info = get_playlist(None)?;
    context.insert("playlists", &info.data);
    context.insert("files", &files.data.files);
    context.insert("files_success", &files.success);
    context.insert("status", &status.data);

    Ok(context)
}

#[test]
fn test_functions() {
    let result = get_verify("test".to_string(), "1".to_string()).unwrap();
    assert_eq!(result, true);
}
