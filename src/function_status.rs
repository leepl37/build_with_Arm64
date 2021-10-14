#![allow(unused_imports)]

use crate::functions::{address_url, call_reqwest};
use crate::json_struct::Status;

pub fn get_status() -> Result<Status, Box<dyn std::error::Error>> {
    let response = call_reqwest()
        .get(address_url("/api/status", None))
        .send()?;
    let return_result: Status = response.json()?;
    Ok(return_result)
}

#[test]
fn test_status() {
    let result = get_status().unwrap();
    assert_eq!(result.data.currentPlaylist, "ttt");
}
