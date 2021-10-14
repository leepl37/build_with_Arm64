#![allow(dead_code)]
#![allow(unused_imports)]
use crate::function_status::get_status;
use crate::functions::{address_url, call_reqwest};
use crate::json_struct::PlayPlayListReturn;
use std::collections::HashMap;

pub enum PlayAndStop {
    Play,
    Stop,
}

pub fn play_and_stop_playlist(
    which: PlayAndStop,
    playlist_name: &str,
) -> Result<PlayPlayListReturn, Box<dyn std::error::Error>> {
    let mut return_json = HashMap::new();
    let response: reqwest::blocking::Response;
    match which {
        PlayAndStop::Play => {
            return_json.insert("play", true);
            response = call_reqwest()
                .post(address_url("/api/play/playlists", Some(playlist_name)))
                .json(&return_json)
                .send()?;
        }
        PlayAndStop::Stop => {
            return_json.insert("stop", true);
            response = call_reqwest()
                .post(address_url("/api/play/playlists", Some(playlist_name)))
                .json(&return_json)
                .send()?;
        }
    }
    let return_object: PlayPlayListReturn = response.json()?;
    Ok(return_object)
}

#[test]
fn test_play_and_stop_playlist() {
    let t = PlayAndStop::Play;
    let playlist = play_and_stop_playlist(t, "gwanho").unwrap();
    let result = get_status().unwrap();
    assert_eq!(result.data.currentPlaylist, playlist.data.name);
}
