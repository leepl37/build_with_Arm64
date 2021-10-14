#![allow(non_snake_case)]
use crate::functions::is_video;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub stat_message: String,
    pub data: Vec<PlayListInfo>,
    pub success: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayListInfo {
    pub settings: Ticker,//
    pub name: String, //
    pub layout: String,  //
    pub assets: Vec<Assets>, //
    pub templatename: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assets {
    pub filename: String,
    pub selected: bool,
    pub isvideo: bool,
    pub duration: String,
    pub fake_name:Option<String>,
}
impl Assets {
    pub fn new(filename: &str, duration_option: Option<String>) -> Assets {
        let mut duration = String::from("");

        match duration_option {
            None => {
                duration = "0".to_string();
            }
            Some(howlong) => {
                if is_video(filename) {
                    duration = "0".to_string()
                } else {
                    duration = howlong;
                }
            }
        }
        Assets {
            filename: filename.to_string(),
            selected: true,
            isvideo: is_video(filename),
            duration,
            fake_name: None
        }
    }
}

// impl Check_Trait for Assets {
//     fn test_check(&self) -> &String {
//         &self.filename
//     }
//
//     fn edit_filename(mut self) {
//         self.filename = self.test_check()[0..10].parse().unwrap()
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Enable {
    pub enable: bool,
}
impl Enable {
    pub fn new() -> Enable {
        Enable { enable: false }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ticker {
    pub enable: Option<bool>,
    pub bannerText: Option<bool>,
    pub behavior: Option<String>,
    pub style: Option<String>,
    pub tickerX: Option<i32>,
    pub tickerY: Option<i32>,
    pub tickerFontSize: Option<i32>,
    pub textSpeed: Option<String>,
    pub messages: Option<String>,
    pub rss: Option<Enable>,
}
impl Ticker {
    pub fn ticker_info(self) -> TickerInfo {
        let msg = self.messages.unwrap().clone();
        // let x2 = msg.trim_end_matches("_");
        let mut split = msg.split("#");
        let color = split.nth(0).unwrap();
        let mut x1 = split.nth(1).unwrap().split("_");
        println!("{:?}", x1);
        let mut vec = Vec::new();
        vec.push(color);
        x1.for_each(|x| vec.push(x));
        println!("{:?}", vec);
        TickerInfo{
            msg1: vec[1].to_string(),
            msg2: vec[2].to_string(),
            msg3: vec[3].to_string(),
            color: vec[0].to_string(),
            fontsize: self.tickerFontSize.unwrap().to_string(),
            speed: self.textSpeed.unwrap().parse().unwrap(),
            check: self.enable.unwrap()
        }

    }

    pub fn new(
        enable: Option<bool>,
        tickerFontSize: Option<i32>,
        textSpeed: Option<String>,
        messages: Option<String>,
    ) -> Ticker {
        Ticker {
            enable,
            bannerText: Some(true),
            behavior: Some("openvg_left".to_string()),
            style: Some("color:red".to_string()),
            tickerX: Some(0),
            tickerY: Some(0),
            tickerFontSize,
            textSpeed,
            messages,
            rss: Some(Enable { enable: false }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleReturnJson {
    pub stat_message: String,
    // pub data:String,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleUploadReturnJson {
    pub stat_message: String,
    pub data: NameSizeType,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameSizeType {
    name: String,
    size: i32,
    r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    pub stat_message: String,
    pub data: FileList,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileList {
    pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub filename: String,
    pub r#type: bool,
    pub fake_name: Option<String>,
}

impl FileInfo {
    pub fn new(name: String) -> FileInfo {
        let info = FileInfo {
            filename: name.clone(),
            r#type: is_video(&name),
            fake_name: None
        };
        info
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub stat_message: String,
    pub data: StatusData,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusData {
    pub diskSpaceUsed: String,
    pub diskSpaceAvailable: String,
    pub duration: i32,
    pub tvStatus: bool,
    pub cecTvStatus: bool,
    pub playlistOn: bool,
    pub currentPlaylist: String,
    pub currentPlayingFile: String,
    pub cpuSerialNumber: String,
    pub playlistsDeployed: Vec<String>,
    pub currentDebugLevel: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayPlayListReturn {
    pub stat_message: String,
    pub data: DataInfo,
    pub success: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DataInfo {
    pub settings: Settings,
    pub name: String,
    pub layout: String,
    pub assets: Vec<Assets>,
    // pub zoneVideoWindow:Option<>,
    pub templateName: String,
    // pub schedule:Option<Vec<_>>,
    // pub labels:Option<Vec<_>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub ticker: Ticker,
    pub ads: Ads,
    pub audio: Audio,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ads {
    pub adCount: i32,
    pub adInterval: i32,
    pub adPlaylist: bool,
}
impl Ads {
    pub fn new() -> Ads {
        Ads {
            adCount: 1,
            adInterval: 60,
            adPlaylist: false,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Audio {
    pub enable: bool,
    pub random: bool,
    pub volume: i32,
}
impl Audio {
    pub fn new() -> Audio {
        Audio {
            enable: false,
            random: false,
            volume: 50,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TickerInfo {
    pub msg1:String,
    pub msg2:String,
    pub msg3:String,
    pub color:String,
    pub fontsize:String,
    pub speed:i32,
    pub check:bool,
}

#[test]
fn json_struct() {
    // let test = "red#0.0#가나다라_ticker_test_";
    // let x2 = test.trim_end_matches("_");
    // let mut split = x2.split("#");
    // let color = split.nth(0).unwrap();
    // let mut x1 = split.nth(1).unwrap().split("_");
    // let mut vec = Vec::new();
    // x1.for_each(|x| vec.push(x));
    // vec.push(color);
    // assert_eq!(vec[0], "가나다라");
    // assert_eq!(vec[1], "ticker");
    // assert_eq!(vec[2], "test");
}