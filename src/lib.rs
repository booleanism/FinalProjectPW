#![feature(async_fn_in_trait)]
pub mod schema;
pub mod crud;

use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthUser {
    pub email: String,
    pub passwd: String
}

#[derive(Deserialize, Serialize)]
pub struct Track {
    pub trackinfo: Vec<TrackInfo>,
    pub current: Info,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TrackInfo {
    id: Option<u64>,
    track_id: Option<u64>,
    pub file: Option<TrackFile>,
    artist: Option<String>,
    pub title: Option<String>,
    encodings_id: Option<u64>,
    license_type: Option<u8>,
    private: Option<bool>,
    track_num: Option<u8>,
    album_preorder: Option<bool>,
    unreleased_track: Option<bool>,
    title_link: Option<String>,
    has_lyrics: Option<bool>,
    has_info: Option<bool>,
    streaming: Option<u8>,
    is_downloadable: Option<bool>,
    has_free_download: Option<bool>,
    free_album_download: Option<bool>,
    duration: Option<f64>,
    lyrics: Option<String>,
    sizeof_lyrics: Option<u64>,
    is_draft: Option<bool>,
    video_source_type: Option<String>,
    video_source_id: Option<String>,
    video_mobile_url: Option<String>,
    video_poster_url: Option<String>,
    video_id: Option<u64>,
    video_caption: Option<bool>,
    video_featured: Option<bool>,
    alt_link: Option<String>,
    encoding_error: Option<bool>,
    encoding_pending: Option<bool>,
    play_count: Option<u8>,
    is_capped: Option<bool>,
    track_license_id: Option<u64>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TrackFile {
    #[serde(rename = "mp3-128")]
    pub mp3_128: String
}

#[derive(Deserialize, Serialize)]
pub struct Info {
    audit: Option<u64>,
    title: Option<String>,
    new_date: Option<String>,
    mod_date: Option<String>,
    publish_date: Option<String>,
    private: Option<bool>,
    killed: Option<bool>,
    download_pref: Option<u8>,
    require_email: Option<bool>,
    is_set_price: Option<bool>,
    set_price: Option<f64>,
    minimum_price: Option<f64>,
    minimum_price_nonzero: Option<f64>,
    require_email_0: Option<bool>,
    artist: Option<String>,
    about: Option<String>,
    credits: Option<String>,
    auto_repriced: Option<String>,
    new_desc_format: Option<u64>,
    band_id: Option<u64>,
    selling_band_id: Option<u64>,
    art_id: Option<u64>,
    download_desc_id: Option<u64>,
    release_date: Option<String>,
    upc: Option<String>,
    purchase_url: Option<String>,
    purchase_title: Option<String>,
    featured_track_id: Option<u64>,
    id: Option<u64>,
    #[serde(rename = "type")]
    tipe: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct LinkTitle {
    pub title: String,
    pub link: String,
}

pub struct DownCamp {
    pub link: String,
}

pub trait Parser {
    fn new(links: String) -> Self;
    fn parse_json(html_code: String) -> serde_json::Result<Track>; 
    async fn fetch(links: String) -> String;
    async fn download(&self) -> Vec<LinkTitle>;
}

impl Parser for DownCamp {
    fn new(link: String) -> Self {
        Self { link: link }
    }

    fn parse_json(html_code: String) -> serde_json::Result<Track> {
        let json_track_start = html_code.find("\"trackinfo").unwrap();
        let json_track_end = html_code.find("}],\"playing_from\"").unwrap();
        let str_track = format!("{}}}]", &html_code[json_track_start..json_track_end]);

        let json_album_start = html_code.find("\"current\":{").unwrap();
        let json_album_end = html_code.find("},\"preorder_count\"").unwrap();
        let str_album = format!("{}}},", &html_code[json_album_start..json_album_end]);
        let str = format!("{{{}{}}}", str_album, str_track);
        let track: Track = match serde_json::from_str(&str) {
            Ok(x) => x,
            Err(e) => panic!("{:#?}", e),
        };

        Ok(track)
    }

    async fn fetch(links: String) -> String {
        let client = reqwest::Client::new();
        let res = client.get(links).send().await.unwrap().text().await.unwrap();

        html_escape::decode_html_entities(&res.as_str().trim()).to_string()
    }

    async fn download(&self) -> Vec<LinkTitle> {
        let html = Self::fetch(self.link.clone()).await;
        let json = Self::parse_json(html).unwrap();
        let mut track = Vec::<LinkTitle>::new();


        for i in &json.trackinfo {
            let a = LinkTitle {
                link: i.file.clone().unwrap().mp3_128,
                title: i.title.clone().unwrap(),

            };
            track.push(a);
        }

        track
    }
}