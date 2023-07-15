#![feature(exact_size_is_empty)]

use std::path::PathBuf;

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
}

// connection string ke database
static CONNSTR: &str = "postgres://postgres:12345@192.168.1.26:5432/bcdl";

/**
 Mengecek apakah user login atau tidak
 */
async fn is_loged_in(cookie: &CookieJar<'_>) -> bool {
    // membuka koneksi ke database
    let pool = Db::new(CONNSTR.to_string()).await;
    // melakukan query ke database
    let pas = pool.select_pass().await;
    // menutup koneksi ke database
    pool.close().await;

    // mengecek cookie
    let cook = match cookie.get_private("key") {
        // mencocokan cookie dengan key "key"
        Some(c) => c,
        // jika tidak cocok maka akan mereturn false
        None => return false
    };
    
    // mengecek apakah password kosong atau tidak
    if !pas.is_empty() {
        // melakukan iterasi ke variabel password
        for i in pas {
            // mencocokan nilai pada cookie yang ada dikirim oleh user
            if cook.value() == i {
                // apabila cocok akan mereturn true
                return true;
            }
        }
        // jika passwprd tidak ditemukan maka false
        return false;
    } else {
        // jika passwprd tidak ditemukan maka false
        return  false;
    }
}

/**
Merespon user apabila mengunjungi /profile.html
 */
#[get("/profile.html")]
async fn profile(cookie: &CookieJar<'_>) -> Template {
    // membuka koneksi ke database
    let pool = Db::new(CONNSTR.to_string()).await;

    // untuk menyimpan data history user dari database
    let mut user_db = Vec::<History>::new();
    // mengecek apakah user login atau tidak melalui cookie
    if is_loged_in(cookie).await {
        // jika user sedang login maka akan melakuakn operasi CRUD untuk melihat history user tersebut
        let user = User { id: cookie.get_private("id").unwrap().value().parse::<i32>().unwrap(), ..Default::default()};
        user_db = pool.select_hist(user).await;
    }
    // menetup koneksi database
    pool.close().await;

    // merender profile.html ke user dengan template engine
    Template::render("profile", context! { list: user_db, logedin: is_loged_in(cookie).await})
}

/**
Merespon user apabila mengunjungi /profile.html
 */
#[get("/login.html")]
async fn login(cookie: &CookieJar<'_>) -> Template {
    // mengecek apakah user tersebut sedang login
    let is_loged_in = is_loged_in(cookie).await;
    // merender profile.html ke user dengan template engine
    Template::render("login", context! { logedin: is_loged_in})
}

/**
Merespon user apabila mengunjungi /signup.html
 */
#[get("/signup.html")]
async fn signup(cookie: &CookieJar<'_>) -> Template {
    // mengecek apakah user tersebut sedang login
    let is_loged_in = is_loged_in(cookie).await;
    // merender profile.html ke user dengan template engine
    Template::render("signup", context! { logedin: is_loged_in})
}

/**
Merespon user apabila mengunjungi /about_us.html
 */
#[get("/about_us.html")]
async fn about(cookie: &CookieJar<'_>) -> Template {
    // mengecek apakah user tersebut sedang login
    let is_loged_in = is_loged_in(cookie).await;
    // merender profile.html ke user dengan template engine
    Template::render("about_us", context! { logedin: is_loged_in})
}

/**
Merespon user apabila mengunjungi /index.html atau /
 */
#[get("/")]
async fn index(cookie: &CookieJar<'_>) -> Template {
    // mengecek apakah user tersebut sedang login
    let is_loged_in = is_loged_in(cookie).await;
    // merender profile.html ke user dengan template engine
    Template::render("index", context! { logedin: is_loged_in, isdownloading: false})
}

/**
Merespon apabila user client merequest gambar
 */
#[get("/<image..>", format = "image/png")]
async fn get_image(image: PathBuf) -> Option<NamedFile> {
    // mencari file gambar pada file directory public
    Some(NamedFile::open(format!("public/{}", image.to_str().unwrap()))
        .await
        .ok()
        .unwrap())
}

/**
Merespon apabila user merequest kode script.js
 */
#[get("/script.js", format = "application/javascript")]
async fn script() -> Option<NamedFile> {
    // mencari file script.js pada directory public
    let file = NamedFile::open(format!("public/{}", "script.js"))
        .await
        .ok()
        .unwrap();
    Some(file)
}

/**
Merespon apabila user merequest kode login.css
 */
#[get("/login.css", format = "text/css")]
async fn css() -> Option<NamedFile> {
    // mencari file login.css pada directory public
    let file = NamedFile::open(format!("public/{}", "login.css"))
        .await
        .ok()
        .unwrap();
    Some(file)
}

/**
Merespon apabila user login
 */
#[post("/auth", format = "application/json", data = "<user>")]
async fn auth(user: Json<AuthUser>, jar: &CookieJar<'_>) -> Status {
    // mengambil data yang dipost oleh user
    let auth_user = user.into_inner();

    // membuat objek user dari data yang sudah dikirimkan  
    let user = User {
        email: auth_user.email,
        ..Default::default()
    };

    // membuka koneksi database
    let pool = Db::new(CONNSTR.to_string()).await;
    // mengambil info login dari database
    let user_db = pool.login(user).await;
    // menutup koneksi ke database
    pool.close().await;

    // melakukan iterasi info login dari database
    for i in user_db {
        // jika password yang berada pada database sama dengan data yang dikirim, maka server akan mengirim cookie dengan "key" dan "id"
        if i.passwd == auth_user.passwd {
            jar.add_private(Cookie::new("key", i.passwd));
            jar.add_private(Cookie::new("id", i.id.to_string()));
            // mereturn nilai ke client accepted / diterima
            return Status::Accepted;
        }
    }
    // mereturn nilai unathorized / tidak mendapatkan izin
    Status::Unauthorized
}

/**
Merespon apabilal user mendaftar
 */
#[post("/reg", format = "application/json", data = "<user>")]
async fn reg(user: Json<AuthUser>) -> Status {
    // mengambil data yang dikirim oleh user
    let auth_user = user.into_inner();

    // jika password yang dikirimkan kosong, maka akan gagal / gagal membuat akun
    if auth_user.passwd == "".to_string() {
        return Status::BadRequest;
    }

    // membuat objek dari data yang dikirimkan
    let user = User {
        email: auth_user.email,
        passwd: auth_user.passwd,
        ..Default::default()
    };

    // membuka koneksi database
    let pool = Db::new(CONNSTR.to_string()).await;
    // melakuakn insert ke database
    pool.reg(user).await;
    // menutup koneksi ke database
    pool.close().await;

    // mereturn value sukses / user telah sibuat
    Status::Created
}

/**
Merespon apabila user melakukan pendownloadan
 */
#[post("/bclink", format = "application/json", data = "<user>")]
async fn bclink(user: Json<BcLinkReq>, cookie: &CookieJar<'_>) -> Template {
    // mengambil data yang dikirim oleh user
    let data = user.into_inner();
    // membuka koneksi ke database
    let pool = Db::new(CONNSTR.to_string()).await;
    
    // jika user login
    if is_loged_in(cookie).await {
        // jika user sedang login maka akan melakuakn operasi CRUD untuk menginsert history user tersebut
        let user = User { id: cookie.get_private("id").unwrap().value().parse::<i32>().unwrap(), ..Default::default()};
        let _ = pool.insert_hist(user, data.bcid.clone()).await;
    }
    // menutup koneksi ke database
    pool.close().await;
    
    // membuat objek dari lagu yang akan didownload
    let a = DownCamp {
        link: data.bcid,
    };

    // membuat variabel untuk menyimpan daftar lagu
    let mut track = Vec::<BcLinkRes>::new();

    // menyimpan daftar lagu
    let link_title = a.download().await;

    // melakukan iterasi daftar lagu
    for i in link_title {
        // membuat objek untuk daftar lagu
        let a = BcLinkRes {
            mp3_link: i.link,
            song_title: i.title,
        };

        // mempush objek ke variabel track
        track.push(a);
    }
    // merender template index
    Template::render("index", context! {isdownloading: true, logedin: is_loged_in(cookie).await, tracklist: track})
}

/**
Merespon apabila user logout
 */
#[get("/logout")]
async fn logout(cookie: &CookieJar<'_>) -> Redirect {
    // mencopot cookie pada client
    cookie.remove_private(Cookie::named("key"));

    Redirect::found(uri!(index))
}

/**
Merespon apabila user mengganti password
 */
#[post("/changepass", format = "application/json", data = "<pass>")]
async fn change_pas(pass: Json<Pass>, cookie: &CookieJar<'_>) -> Status {
    // mengambil data yang dikirim oleh user
    let a = pass.into_inner();
    
    if is_loged_in(cookie).await {
        // jika user login, maka akan membuka koneksi ke database
        let pool = Db::new(CONNSTR.to_string()).await;
        // membuat objek untuk user berdasarkan id cookie yang dikirimkan
        let user = User { id: cookie.get_private("id").unwrap().value().parse::<i32>().unwrap(), passwd: a.npasswd, ..Default::default()};
        // melakukan operasi CRUD untuk mengupdate password user
        let _ = pool.change_pas(user).await;
        // menutup koneksi ke database
        pool.close().await;
        // menghapus cookie pada user
        cookie.remove_private(Cookie::named("id"));
        cookie.remove_private(Cookie::named("key"));
        // mereturn ok / berhasil
        return Status::Ok
    }
    // mereturn conflict / gagal
    Status::Conflict
}

/**
Entry point dari program ini
 */
#[launch]
async fn rocket() -> _ {
    // memount / merutekan webserver
    rocket::build().mount("/", routes![
        index, login, signup, about, 
        auth, reg, bclink, script, css, 
        logout, profile, change_pas, get_image]).attach(Template::fairing())
}