#![feature(proc_macro_hygiene, decl_macro)]

use std::path::PathBuf;

use rocket::{fs::NamedFile, serde::{Deserialize, Serialize, json::{serde_json::json, Json, Value}}, data::FromData, http::{Cookie, CookieJar}};

#[macro_use] extern crate rocket;
extern crate serde;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct AccountInfo {
    id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct LoginRes {
    status: i16,
    // message: String
    // data: AccountInfo
}

#[derive(Debug)]
enum LoginError {
    InvalidData,
    UsernameDoesNotExist,
    WrongPassword
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    // fs::read_to_string("public/index.html").unwrap()
    NamedFile::open("public/index.html").await.ok()
}

#[get("/<file..>")]
async fn index_ex(file: PathBuf) -> Option<NamedFile> {
    // fs::read_to_string("public/index.html").unwrap()
    NamedFile::open(format!("public/{}", file.to_str().unwrap())).await.ok()
}

#[post("/auth", format="json", data="<account_info>")]
async fn auth(account_info: Json<AccountInfo>, cookie: &CookieJar<'_>) -> Option<rocket::serde::json::Value> {
    let res = LoginRes {
        // data: AccountInfo { id: account_info.id.to_string() },
        status: 200
    };
    
    // bikin fungsi untuk mengolah account_info yang mereturn value cookie (dalam base64 (hasil dari email dan sandi yang sudah dikonfirmasi pada database))
    cookie.add(Cookie::new("id", account_info.id.to_string()));

    Some(json!(res))
}

#[get("/login")]
async fn login(cookie: &CookieJar<'_>) -> Value {
    let cook = cookie.get("id");
    let mut id = "";
    cook.map(|x| id = x.value());

    if id == "1" {
        json!(AccountInfo {
            id: id.to_string()
        })
    } else {
        json!(LoginRes {
            status: 403
        })
    }

}


#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index, login, auth, index_ex])
}