#![allow(dead_code)]
#![allow(unused_imports)]
use crate::functions::{address_url, call_reqwest};
use crate::json_struct::{Assets, Playlist, SimpleReturnJson, PlayPlayListReturn};
use actix_web::HttpResponse;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

pub fn get_playlist(playlist_name: Option<String>) -> Result<Playlist, Box<dyn std::error::Error>> {
    let result = call_reqwest()
        .get(address_url("/api/playlists", None))
        .send()?;
    let mut playlist: Playlist = result.json()?;
    println!("{:?}", playlist);
    if let Some(name) = playlist_name {
        playlist.data.retain(|info| info.name == name);
        println!("{:?}", playlist);
        // let option = playlist.data.pop().unwrap();
    }
    playlist.data.retain(|info| info.name != "TV_OFF");
    Ok(playlist)
}



//ticker message 출력하기.
pub fn get_the_playlist(playlist_name: String) -> Result<PlayPlayListReturn, Box<dyn std::error::Error>> {
    let result = call_reqwest()
        .get(address_url("/api/playlists", Option::from(playlist_name.as_str())))
        .send()?;
    let mut playlist: PlayPlayListReturn = result.json()?;
    println!("{:?}", playlist);
    Ok(playlist)
}



pub fn create_playlist(name: &str) -> Result<SimpleReturnJson, Box<dyn std::error::Error>> {
    //중복생성처리 안되어 있음.
    let mut form = HashMap::new();
    form.insert("file", name);
    let response = call_reqwest()
        .post(address_url("/api/playlists", None))
        .form(&form)
        .send()?;
    let return_object: SimpleReturnJson = response.json()?;
    Ok(return_object)
}

pub fn delete_playlist(name: &str) -> Result<SimpleReturnJson, Box<dyn std::error::Error>> {
    //중복제거 처리 안되어 있음 & 없는 파일도 지워졌다는 리턴값 나옴.
    let playlist_name_formatting_json = format!("__{}.json", name);
    let response = call_reqwest()
        .delete(address_url(
            "/api/files",
            Some(playlist_name_formatting_json.as_str()),
        ))
        .send()?;
    let return_object: SimpleReturnJson = response.json()?;
    Ok(return_object)
}

pub enum AddOrRemove {
    Add(Assets),
    Remove(String),
    Edit(Vec<Assets>),
    RemoveAll,
}

pub fn update_playlist(
    file_name: AddOrRemove,
    playlist_name: &str,
) -> Result<SimpleReturnJson, Box<dyn std::error::Error>> {
    let mut get_playlists = get_playlist(Some(playlist_name.to_string()))?;
    let mut updated_asset = HashMap::new();

    match file_name {
        AddOrRemove::Add(assets) => {
            let mut info = get_playlists.data.pop().unwrap();
            info.assets.push(assets);
            updated_asset.insert("assets", info.assets);
        }
        AddOrRemove::Remove(file_name) => {
            let mut info = get_playlists.data.pop().unwrap();
            info.assets.retain(|f| f.filename != file_name);
            updated_asset.insert("assets", info.assets);
        }
        AddOrRemove::RemoveAll => {
            let mut info = get_playlists.data.pop().unwrap();
            info.assets
                .retain(|f| f.filename == "모든 파일 지우기..........");
            updated_asset.insert("assets", info.assets);
        }
        AddOrRemove::Edit(assets) => {
            let mut info = get_playlists.data.pop().unwrap();
            updated_asset.insert("assets", assets);
        }
    }
    let response = call_reqwest()
        .post(address_url("/api/playlists", Some(playlist_name)))
        .json(&updated_asset)
        .send()?;
    let result: SimpleReturnJson = response.json()?;

    Ok(result)
}

pub fn update_playlist_all_for_one(
    split_file_name_list: String,
    split_duration_list: String,
    playlist_name: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let split_file_name = split_file_name_list.split("|");
    let split_duration = split_duration_list.split("|");

    let mut file_name: Vec<String> = Vec::new();
    let mut duration = Vec::new();
    for n in split_file_name {
        if !n.is_empty() {
            file_name.push(n.trim().to_string());
        }
    }
    for n in split_duration {
        if !n.is_empty() {
            let x = n.trim();
            let mut some_return = None;
            if x.starts_with("f") {
                some_return = None;
            } else {
                let count = x;
                some_return = Some(count.to_string());
            }
            duration.push(some_return);
        }
    }
    let mut assets_vec = Vec::new();
    for n in file_name {
        let x = duration.first().unwrap().clone();
        duration.remove(0);
        let assets = Assets::new(n.as_str(), x);
        assets_vec.push(assets)
    }
    let result = update_playlist(AddOrRemove::RemoveAll, &*playlist_name);
    let result1 = update_playlist(AddOrRemove::Edit(assets_vec), &*playlist_name);
    if result?.success == result1?.success {
        Ok(true)
    } else {
        Ok(false)
    }
}



// pub trait Check_Trait {
//     fn test_check(&self) -> &String;
//
//     fn edit_filename(&mut self) -> ();
//
// }

#[test]
fn test_playlist() {
    // fn test<T:Check_Trait>(mut vec: Vec<T>)  {
    //     for n in vec.iter_mut() {
    //         if n.test_check().len() > 10{
    //             let string = n.test_check()[0..10].to_string();
    //             n.edit_filename(string);
    //             println!("{:?}", n.test_check());
    //         }
    //     }
    // }
    // let mut get_the_playlist = get_playlist(Some("ITRO".to_string())).unwrap();
    // let mut info = get_the_playlist.data.pop().unwrap().assets;

}
