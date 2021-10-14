#![allow(unused_imports)]
use crate::functions::{address_url, call_reqwest, set_ticker_message};
use crate::json_struct::{Ads, Audio, Enable, Playlist, Settings, SimpleReturnJson, Ticker};
use std::collections::HashMap;

pub fn setting_ticker(
    ticker_setting: Ticker,
    playlist_name: &str,
) -> Result<SimpleReturnJson, Box<dyn std::error::Error>> {
    let set = Settings {
        ticker: ticker_setting,
        ads: Ads::new(),
        audio: Audio::new(),
    };
    let mut settings = HashMap::new();
    settings.insert("settings", set);

    let response = call_reqwest()
        .post(address_url("/api/playlists", Some(playlist_name)))
        .json(&settings)
        .send()?;

    let return_object: SimpleReturnJson = response.json()?;
    Ok(return_object)
}

#[test]
fn test_ticker() {
    // let message = set_ticker_message(
    //     "test~테스트1".to_string(),
    //     "test~테스트2".to_string(),
    //     "test~테스트3".to_string(),
    //     "red".to_string(),
    // );
    let message = set_ticker_message(
        "test~테스트1".to_string(),
        "test~테스트2".to_string(),
        "".to_string(),
        "red".to_string(),
    );

    let ticker1 = Ticker::new(Some(true), Some(30), Some("3".to_string()), Some(message));

    let result = setting_ticker(ticker1, "ITRO");
    let info = result.unwrap();
    println!("{:?}", info);
    assert_eq!(info.success, true)
}
