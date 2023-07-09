#![feature(exact_size_is_empty)]

use FinalProjectPW::{AuthUser, crud::{Db, Koneksi}, schema::{User, History}};

use FinalProjectPW::{DownCamp, Parser};
use rocket::{
    get, 
    fs::NamedFile, 
    serde::{
        json::Json, 
        Serialize, 
        Deserialize
    }, 
    http::{CookieJar, Cookie, Status}, 
    response::Redirect
};
use rocket_dyn_templates::{Template, context};

#[macro_use] 
extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct BcLinkReq {
    bcid: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct BcLinkRes {
    song_title: String,
    mp3_link: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Pass {
    cpasswd: String,
    npasswd: String,
    // id: i32,
}

static CONNSTR: &str = "postgres://postgres:12345@192.168.18.252:5432/bcdl";

async fn is_loged_in(cookie: &CookieJar<'_>) -> bool {
    let pool = Db::new(CONNSTR.to_string()).await;
    let pas = pool.select_pass().await;
    println!("{:?}", pas);
    pool.close().await;

    let cook = match cookie.get_private("key") {
        Some(c) => c,
        None => return false
    };
    
    if !pas.is_empty() {
        for i in pas {
            if cook.value() == i {
                return true;
            }
        }
        return false;
    } else {
        return  false;
    }
}

#[get("/profile.html")]
async fn profile(cookie: &CookieJar<'_>) -> Template {
    let pool = Db::new(CONNSTR.to_string()).await;

    let mut user_db = Vec::<History>::new();
    if is_loged_in(cookie).await {
        let user = User { id: cookie.get_private("id").unwrap().value().parse::<i32>().unwrap(), ..Default::default()};
        user_db = pool.select_hist(user).await;
    }
    pool.close().await;

    Template::render("profile", context! { list: user_db, logedin: is_loged_in(cookie).await})
}

#[get("/login.html")]
async fn login(cookie: &CookieJar<'_>) -> Template {
    let is_loged_in = is_loged_in(cookie).await;
    Template::render("login", context! { logedin: is_loged_in})
}

#[get("/signup.html")]
async fn signup(cookie: &CookieJar<'_>) -> Template {
    let is_loged_in = is_loged_in(cookie).await;
    Template::render("signup", context! { logedin: is_loged_in})
}

#[get("/about_us.html")]
async fn about(cookie: &CookieJar<'_>) -> Template {
    let is_loged_in = is_loged_in(cookie).await;
    Template::render("about_us", context! { logedin: is_loged_in})
}

#[get("/")]
async fn index(cookie: &CookieJar<'_>) -> Template {
    let is_loged_in = is_loged_in(cookie).await;
    Template::render("index", context! { logedin: is_loged_in, isdownloading: false})
}

// #[get("/<file..>")]
// async fn catch_all(file: PathBuf, cookie: &CookieJar<'_>) -> Template {
//     let name_tera = file.to_str().unwrap().replace(".html", "");
//     println!("{}", name_tera);
//     let is_loged_in = is_loged_in(cookie).await;
//     Template::render(name_tera, context! { logedin: is_loged_in})
// }

#[get("/script.js", format = "application/javascript")]
async fn script() -> Option<NamedFile> {
    let file = NamedFile::open(format!("public/{}", "script.js"))
        .await
        .ok()
        .unwrap();
    Some(file)
}

#[get("/login.css", format = "text/css")]
async fn css() -> Option<NamedFile> {
    let file = NamedFile::open(format!("public/{}", "login.css"))
        .await
        .ok()
        .unwrap();
    Some(file)
}


#[post("/auth", format = "application/json", data = "<user>")]
async fn auth(user: Json<AuthUser>, jar: &CookieJar<'_>) -> Status {
    let auth_user = user.into_inner();

    let user = User {
        email: auth_user.email,
        ..Default::default()
    };

    let pool = Db::new(CONNSTR.to_string()).await;
    let user_db = pool.login(user).await;
    pool.close().await;

    for i in user_db {
        if i.passwd == auth_user.passwd {
            jar.add_private(Cookie::new("key", i.passwd));
            jar.add_private(Cookie::new("id", i.id.to_string()));
            return Status::Accepted;
        }
    }

    Status::Unauthorized
}

#[post("/reg", format = "application/json", data = "<user>")]
async fn reg(user: Json<AuthUser>) -> Status {
    let auth_user = user.into_inner();

    if auth_user.passwd == "".to_string() {
        return Status::BadRequest;
    }

    let user = User {
        email: auth_user.email,
        passwd: auth_user.passwd,
        ..Default::default()
    };

    let pool = Db::new(CONNSTR.to_string()).await;
    pool.reg(user).await;
    pool.close().await;

    Status::Created
}

/*
* panggil fungsi pada library bcdownloader
* 
 */
#[post("/bclink", format = "application/json", data = "<user>")]
async fn bclink(user: Json<BcLinkReq>, cookie: &CookieJar<'_>) -> Template {
    let data = user.into_inner();
    let pool = Db::new(CONNSTR.to_string()).await;
    
    if is_loged_in(cookie).await {
        let user = User { id: cookie.get_private("id").unwrap().value().parse::<i32>().unwrap(), ..Default::default()};
        let _ = pool.insert_hist(user, data.bcid.clone()).await;
    }
    pool.close().await;
    

    let a = DownCamp {
        link: data.bcid,
    };

    let mut track = Vec::<BcLinkRes>::new();

    let link_title = a.download().await;

    for i in link_title {
        let a = BcLinkRes {
            mp3_link: i.link,
            song_title: i.title,
        };

        track.push(a);
    }
    Template::render("index", context! {isdownloading: true, logedin: is_loged_in(cookie).await, tracklist: track})
}

#[get("/logout")]
async fn logout(cookie: &CookieJar<'_>) -> Redirect {
    cookie.remove_private(Cookie::named("key"));

    Redirect::found(uri!(index))
}

#[post("/changepass", format = "application/json", data = "<pass>")]
async fn change_pas(pass: Json<Pass>, cookie: &CookieJar<'_>) -> Status {
    let a = pass.into_inner();
    
    if is_loged_in(cookie).await {
        let pool = Db::new(CONNSTR.to_string()).await;
        let user = User { id: cookie.get_private("id").unwrap().value().parse::<i32>().unwrap(), passwd: a.npasswd, ..Default::default()};
        let _ = pool.change_pas(user).await;
        pool.close().await;
        cookie.remove_private(Cookie::named("id"));
        cookie.remove_private(Cookie::named("key"));
        return Status::Ok
    }

    Status::Conflict
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index, login, signup, about, auth, reg, bclink, script, css, logout, profile, change_pas]).attach(Template::fairing())
}