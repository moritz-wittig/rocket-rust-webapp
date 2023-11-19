use rocket::{http::{Status, uri::Origin}, response::Redirect, serde::json::{Value, json }};
use rocket::State;
use reqwest;
use reqwest::Client;

#[macro_use]
extern crate rocket;

const TAURI_RELEASES_PREFIX: Origin<'static> = uri!("/tauri-releases");

#[get("/")]
fn index() -> Redirect{
    let msg: Option<&str> = None;
    Redirect::to(uri!(TAURI_RELEASES_PREFIX, google_keep_desktop_api("windows-x86_64", "v1.0.14", msg)))
}

const GOOGLE_KEEP_DESKTOP_REPO: &str = "elibroftw/google-keep-desktop-app";

async fn get_latest_release(client: &State<Client>, repo: &str) -> Result<Value, reqwest::Error>{
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");
    let response = client.get(&url).send().await?;
    let github_release = response.json::<Value>().await?;
    Ok(github_release)
}

#[get("/google-keep-desktop/<_platform>/<current_version>?<msg>")]
async fn google_keep_desktop_api(_platform: &str, current_version: &str, msg: Option<&str>, client: &State<  Client>) -> Result<Value, Status>{
    // Status::NoContent
    if let Some(msg) = msg{
        println!("{msg}");
        return Err(Status::NoContent);
    }
    get_latest_release(client, GOOGLE_KEEP_DESKTOP_REPO).await.or (Err(Status::NoContent))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(reqwest::Client::builder()
            .user_agent("reqwest")
            .build()
            .unwrap()
        )
        .mount("/", routes![index])
        .mount(TAURI_RELEASES_PREFIX, routes![google_keep_desktop_api])
}