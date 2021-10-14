#![allow(unused_imports)]
use crate::function_files::{delete_file, get_files, get_files_info, upload_file};
use crate::function_play_and_stop::{play_and_stop_playlist, PlayAndStop};
use crate::function_playlist::{create_playlist, delete_playlist, get_playlist, update_playlist, update_playlist_all_for_one, get_the_playlist};
use crate::function_status::get_status;
use crate::function_ticker::setting_ticker;
use crate::functions::{get_context_for_main, get_verify, set_ticker_message};
use crate::html_struct::{
    CreatePlaylistForm, FileDeleteForm, FileEditForm, FooterFormPlaylist, LoginForm, TickerForm,
};
use crate::json_struct::{Files, SimpleReturnJson, SimpleUploadReturnJson, Ticker, TickerInfo};
use crate::render_function::js_file_list_to_rs_file_list_then_insert_playlist;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use std::error::Error;
use tera::{Context, Tera};
use std::env::split_paths;

#[get("/")]
pub async fn login(tera: web::Data<Tera>, id: Identity) -> HttpResponse {
    match id.identity() {
        None => {
            let mut data = Context::new();
            data.insert("err", "none");
            let rendered = tera.render("m_login.html", &data).unwrap();
            HttpResponse::Ok().body(rendered)
        }
        Some(user_name) => {
            return match get_context_for_main() {
                Ok(mut context) => {
                    context.insert("err", "none");
                    let rendered = tera.render("m_main.html", &context).unwrap();
                    HttpResponse::Ok().body(rendered)
                }
                Err(err) => {
                    let mut data = Context::new();
                    let err_str = format!("문제발생: {}", err.to_string());
                    data.insert("err", &err_str.to_string());
                    let rendered = tera.render("m_login.html", &data).unwrap();
                    HttpResponse::Ok().body(rendered)
                }
            }
        }
    }
}
#[post("/")]
pub async fn login_process(
    form: web::Form<LoginForm>,
    tera: web::Data<Tera>,
    id: Identity,
) -> HttpResponse {
    println!("{:?}", "로그인 POST");
    let mut data = Context::new();
    return match get_verify(form.name.clone(), form.pass.clone()) {
        Ok(bool) => {
            if bool {
                id.remember(form.name.clone());
                HttpResponse::Found().header("location", "/").finish()
            } else {
                data.insert("err", "일치하는 정보가 존재하지 않습니다.");
                let rendered = tera.render("m_login.html", &data).unwrap();
                HttpResponse::Ok().body(rendered)
            }
        }
        Err(err) => {
            data.insert("err", &err.to_string());
            let rendered = tera.render("m_login.html", &data).unwrap();
            HttpResponse::Ok().body(rendered)
        }
    };
}

#[post("/upload")]
pub async fn upload(mut parts: awmp::Parts, tera: web::Data<Tera>, id: Identity) -> HttpResponse {
    for f in parts.files.take("file") {
        let file_name = f.sanitized_file_name().replace(" ", "");
        // let file_name = f.sanitized_file_name();
        let file_path = f.as_ref().path();
        match upload_file(file_path, file_name.to_string()) {
            Ok(ok) => {
                println!("파일이 성공적으로 업로드 되었습니다.");
            }
            Err(err) => {
                let err_str = err.to_string();
                let msg = format!(
                    "파일 업로드에 실패하였습니다. 관리자에게 문의해주세요 :)  에러코드 : {}",
                    err_str
                );
                return match get_context_for_main() {
                    Ok(mut context) => {
                        context.insert("err", &msg);
                        let rendered = tera.render("m_main.html", &context).unwrap();
                        HttpResponse::Ok().body(rendered)
                    }
                    Err(err) => {
                        let mut data = Context::new();
                        let err_str = format!("문제발생: {}", err.to_string());
                        data.insert("err", &err_str.to_string());
                        let rendered = tera.render("m_login.html", &data).unwrap();
                        HttpResponse::Ok().body(rendered)
                    }
                };
            }
        }
    }
    let context1 = Context::new();
    let rendered = tera.render("alert_test.html", &context1).unwrap();
    HttpResponse::Ok().body(rendered)
    // HttpResponse::Found().header("location", "/").finish()
}

#[post("/delete")]
pub async fn delete(form: web::Form<FileDeleteForm>, tera: web::Data<Tera>) -> HttpResponse {
    println!("{:?}", form);
    let file_list = form.file_name.clone();
    let split: Vec<&str> = file_list.split(",").collect();
    for n in split.iter() {
        match delete_file(n) {
            Ok(ok) => {
                println!("{}", "파일이 잘 삭제 되었습니다.");
            }
            Err(err) => {
                return match get_context_for_main() {
                    Ok(mut context) => {
                        context.insert("err", &err.to_string());
                        let rendered = tera.render("m_main.html", &context).unwrap();
                        HttpResponse::Ok().body(rendered)
                    }
                    Err(err) => {
                        let mut data = Context::new();
                        let err_str = format!("문제발생: {}", err.to_string());
                        data.insert("err", &err_str.to_string());
                        let rendered = tera.render("m_login.html", &data).unwrap();
                        HttpResponse::Ok().body(rendered)
                    }
                };
            }
        }
    }
    HttpResponse::Found().header("location", "/").finish()
}

#[get("/create")]
pub async fn create(tera: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    match get_files() {
        Ok(file) => {
            let files = get_files().unwrap();


            let mut files = get_files_info(file.data.files);
            for n in files.iter_mut() {
                if n.filename.len() > 20 {
                    let mut slice_name = n.filename[0..20].to_string();
                    slice_name.push_str("...");
                    n.fake_name = Option::from(slice_name);
                }else{
                    n.fake_name = Option::from(n.filename.clone());
                }
            }

            context.insert("files", &files);

        },
        Err(err) => {
            return match get_context_for_main() {
                Ok(mut context) => {
                    context.insert("err", &err.to_string());
                    let rendered = tera.render("m_main.html", &context).unwrap();
                    HttpResponse::Ok().body(rendered)
                }
                Err(err) => {
                    let mut data = Context::new();
                    let err_str = format!("문제발생: {}", err.to_string());
                    data.insert("err", &err_str.to_string());
                    let rendered = tera.render("m_login.html", &data).unwrap();
                    HttpResponse::Ok().body(rendered)
                }
            }
        }
    }
    let rendered = tera.render("create.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/create_add_edit")]
pub async fn create_post(
    form: web::Form<CreatePlaylistForm>,
    tera: web::Data<Tera>,
) -> HttpResponse {
    println!("{:?}", form);
    if form.playlist_name == "" {
        return match get_context_for_main() {
            Ok(mut context) => {
                context.insert("err", "플레이리스트 이름이 입력되지 않았습니다.");
                let rendered = tera.render("m_main.html", &context).unwrap();
                HttpResponse::Ok().body(rendered)
            }
            Err(err) => {
                let mut data = Context::new();
                let err_str = format!("문제발생: {}", err.to_string());
                data.insert("err", &err_str.to_string());
                let rendered = tera.render("m_login.html", &data).unwrap();
                HttpResponse::Ok().body(rendered)
            }
        };
    }
    if form.form_type == "edit" {
        let files_name = form.checked_file.clone();
        js_file_list_to_rs_file_list_then_insert_playlist(
            form.playlist_name.clone(),
            files_name,
            form.form_type.clone(),
        )
    }
    match create_playlist(&form.playlist_name) {
        Ok(ok) => {
            let files_name = form.checked_file.clone();
            if !files_name.is_empty() {
                js_file_list_to_rs_file_list_then_insert_playlist(
                    form.playlist_name.clone(),
                    files_name,
                    form.form_type.clone(),
                )
            }
        }
        Err(err) => {
            let err_str = format!(
                "플레이리스트 생성 중에 에러가 발생하였습니다. Err Code: {}",
                err.to_string()
            );
            return match get_context_for_main() {
                Ok(mut context) => {
                    context.insert("err", &err_str);
                    let rendered = tera.render("m_main.html", &context).unwrap();
                    HttpResponse::Ok().body(rendered)
                }
                Err(err) => {
                    let mut data = Context::new();
                    let err_str = format!("문제발생: {}", err.to_string());
                    data.insert("err", &err_str.to_string());
                    let rendered = tera.render("m_login.html", &data).unwrap();
                    HttpResponse::Ok().body(rendered)
                }
            };
        }
    }
    HttpResponse::Found().header("location", "/").finish()
}

#[post("/playlist_status")]
pub async fn play_and_stop_playlist_post(
    form: web::Form<FooterFormPlaylist>,
    tera: web::Data<Tera>,
) -> HttpResponse {
    let mut context = Context::new();
    println!("{:?}", form);
    if form.status == "재생" {
        play_and_stop_playlist(PlayAndStop::Play, form.playlist_name.as_str());
    } else if form.status == "삭제" {
        delete_playlist(form.playlist_name.as_str());
    } else if form.status == "문구"{
        let ticker = get_the_playlist(form.playlist_name.to_string().clone()).unwrap().data.settings.ticker;
        let get_the_playlist = get_playlist(Some(form.playlist_name.clone())).unwrap();
        context.insert("playlist_name", &get_the_playlist.data[0].name);
        let info = ticker.ticker_info();
        let mut ticker_info = TickerInfo{
            msg1: "".to_string(),
            msg2: "".to_string(),
            msg3: "".to_string(),
            color: "black".to_string(),
            fontsize: "30".to_string(),
            speed: 1,
            check: false
        };
        if info.check {
            ticker_info = info;
        }
        context.insert("ticker_info", &ticker_info);


        let rendered = tera.render("test.html", &context).unwrap();
        return HttpResponse::Ok().body(rendered)
    } else if form.status == "정지" {
        play_and_stop_playlist(PlayAndStop::Stop, form.playlist_name.as_str());
    } else if form.status == "수정" {
        return match get_files() {
            Ok(file) => {
                let mut get_the_playlist = get_playlist(Some(form.playlist_name.to_string())).unwrap();
                let mut info = get_the_playlist.data.pop().unwrap().assets;
                for n in info.iter_mut() {
                    if n.filename.len() > 20 {
                        let mut slice_name = n.filename[0..20].to_string();
                        slice_name.push_str("...");
                        n.fake_name = Option::from(slice_name);
                    }else{
                        n.fake_name = Option::from(n.filename.clone());
                    }
                }


                let mut files = get_files_info(file.data.files);
                for n in files.iter_mut() {
                    if n.filename.len() > 20 {
                        let mut slice_name = n.filename[0..20].to_string();
                        slice_name.push_str("...");
                        n.fake_name = Option::from(slice_name);
                    }else{
                        n.fake_name = Option::from(n.filename.clone());
                    }
                }

                context.insert("files", &files);
                context.insert("playlist", &info);
                context.insert("playlist_name", &form.playlist_name);
                let rendered = tera.render("test2.html", &context).unwrap();
                HttpResponse::Ok().body(rendered)
            }
            Err(err) => {
                let err_str = format!(
                    "플레이 리스트를 불러오는 중에 문제가 발생하였습니다. Err_Code: {}",
                    err.to_string()
                );
                match get_context_for_main() {
                    Ok(mut context) => {
                        context.insert("err", &err_str);
                        let rendered = tera.render("m_main.html", &context).unwrap();
                        HttpResponse::Ok().body(rendered)
                    }
                    Err(err) => {
                        let mut data = Context::new();
                        let err_str = format!("문제발생: {}", err.to_string());
                        data.insert("err", &err_str.to_string());
                        let rendered = tera.render("m_login.html", &data).unwrap();
                        HttpResponse::Ok().body(rendered)
                    }
                }
            }
        };
    }
    HttpResponse::Found().header("location", "/").finish()
}

#[get("/test")]
pub async fn test(tera: web::Data<Tera>) -> HttpResponse {
    println!("적용");
    let mut context = Context::new();
    let ticker = get_the_playlist("ITRO".to_string()).unwrap().data.settings.ticker;
    let get_the_playlist = get_playlist(Some("ITRO".to_string())).unwrap();
    context.insert("playlist_name", &get_the_playlist.data[0].name);
    let info = ticker.ticker_info();
    let mut ticker_info = TickerInfo{
        msg1: "".to_string(),
        msg2: "".to_string(),
        msg3: "".to_string(),
        color: "black".to_string(),
        fontsize: "30".to_string(),
        speed: 1,
        check: false
    };
    if info.check {
        ticker_info = info;
    }


    context.insert("ticker_info", &ticker_info);

    let rendered = tera.render("test.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}
#[get("/test2")]
pub async fn test2(tera: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    let file = get_files().unwrap();
    let mut get_the_playlist = get_playlist(Some("ITRO".to_string())).unwrap();
    let playlist_name = get_the_playlist.data[0].name.clone();
    // context.insert("files", &files.data.files);

    let mut get_the_playlist = get_playlist(Some("ITRO".to_string())).unwrap();
    let mut info = get_the_playlist.data.pop().unwrap().assets;
    for n in info.iter_mut() {
        if n.filename.len() > 20 {
            let mut slice_name = n.filename[0..20].to_string();
            slice_name.push_str("...");
            n.fake_name = Option::from(slice_name);
        }else{
            n.fake_name = Option::from(n.filename.clone());
        }
    }


    let mut files = get_files_info(file.data.files);
    for n in files.iter_mut() {
        if n.filename.len() > 20 {
            let mut slice_name = n.filename[0..20].to_string();
            slice_name.push_str("...");
            n.fake_name = Option::from(slice_name);
        }else{
            n.fake_name = Option::from(n.filename.clone());
        }
    }

    context.insert("files", &files);
    context.insert("playlist", &info);
    context.insert("playlist_name", &playlist_name);

    let rendered = tera.render("test2.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/edit_playlist")]
pub async fn edit_playlist_post(form: web::Form<FileEditForm>) -> HttpResponse {
    let result = update_playlist_all_for_one(
        form.playlist.clone(),
        form.durationlist.clone(),
        form.playlist_name.to_string(),
    );
    //에러체크
    let status = get_status().unwrap().data.currentPlaylist;
    if status.starts_with(form.playlist_name.as_str()){
        play_and_stop_playlist(PlayAndStop::Stop, form.playlist_name.as_str());
        play_and_stop_playlist(PlayAndStop::Play, form.playlist_name.as_str());
    }

    HttpResponse::Found().header("location", "/").finish()
}

#[post("/create_playlist")]
pub async fn create_playlist_post(form: web::Form<FileEditForm>) -> HttpResponse {
    let created = create_playlist(form.playlist_name.clone().as_str());
    let result = update_playlist_all_for_one(
        form.playlist.clone(),
        form.durationlist.clone(),
        form.playlist_name.to_string(),
    );
    HttpResponse::Found().header("location", "/").finish()
}


#[post("/ticker_setting")]
pub async fn ticker_setting_post(mut form: web::Form<TickerForm>) -> HttpResponse {
    println!("{:?}", form);
    if form.check1.is_some()||form.check2.is_some()||form.check3.is_some() {
        form.check = Some("true".to_string());
    }
    let message = set_ticker_message(
        form.msg1.to_string(),
        form.msg2.to_string(),
        form.msg3.to_string(),
        form.color.to_string(),
    );
    let fontsize: i32 = form.fontsize.parse().unwrap();
    let ticker_set = Ticker::new(
        Some(form.check.is_some()),
        Some(fontsize),
        Some(form.speed.to_string()),
        Some(message),
    );
    let result = setting_ticker(ticker_set, form.playlist_name.as_str());
    let status = get_status().unwrap().data.currentPlaylist;
    if status.starts_with(form.playlist_name.as_str()){
        play_and_stop_playlist(PlayAndStop::Stop, form.playlist_name.as_str());
        play_and_stop_playlist(PlayAndStop::Play, form.playlist_name.as_str());
    }
    HttpResponse::Found().header("location", "/").finish()
}
#[get("/testtest")]
pub async fn testtest(tera: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    let files = get_files().unwrap();

    let rendered = tera.render("testtest.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}