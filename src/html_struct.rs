use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FileDeleteForm {
    pub file_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePlaylistForm {
    pub playlist_name: String,
    pub checked_file: String,
    pub form_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FooterFormPlaylist {
    pub playlist_name: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginForm {
    pub name: String,
    pub pass: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileEditForm {
    pub playlist: String,
    pub durationlist: String,
    pub playlist_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TickerForm {
    pub playlist_name:String,
    pub msg1: String,
    pub msg2: String,
    pub msg3: String,
    pub check1: Option<String>,
    pub check2: Option<String>,
    pub check3: Option<String>,
    pub check: Option<String>,
    pub color: String,
    pub fontsize: String,
    pub speed: i32,
}
