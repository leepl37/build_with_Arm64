#![allow(unused_imports)]
use crate::function_playlist::{update_playlist, AddOrRemove};
use crate::functions::{address_url, call_reqwest};
use crate::html_struct::CreatePlaylistForm;
use crate::json_struct::Assets;
use std::collections::HashMap;
use std::num::ParseIntError;

pub fn js_file_list_to_rs_file_list_then_insert_playlist(
    playlist_name: String,
    mut files_name: String,
    type_check: String,
) {
    let mut asset_vec = Vec::new();
    if !files_name.is_empty() {
        let i = files_name.len();
        files_name.remove(i - 1);
        let mut duration: i32 = 0;
        let mut file_name = String::from("");
        let splitted_file_list = files_name.split(",");
        for n in splitted_file_list.into_iter() {
            let split = n.split(":");
            for n in split.into_iter() {
                match n.parse::<i32>() {
                    Ok(dura) => {
                        duration = dura;
                    }
                    Err(err) => {
                        file_name = n.to_string();
                    }
                }
            }
            let assets_ = Assets::new(file_name.as_str(), Some(duration.to_string()));
            asset_vec.push(assets_)
        }
    }
    println!("type_check: {}", type_check);
    if type_check.contains("edit") {
        println!("전체 파일 지우기");
        update_playlist(AddOrRemove::RemoveAll, &playlist_name);
    }
    for a in asset_vec {
        update_playlist(AddOrRemove::Add(a), &playlist_name);
    }
}
#[test]
fn test_render_function() {
    let string = String::from("");
    assert_eq!(string.is_empty(), true);
}
