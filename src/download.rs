use crate::error;
use reqwest;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;

//the result type can hold either a value of T or an Error.
//Box is used to box the error ,allows it to be sent over threads and shared between threads safely.
//Same functionality is done by Send and Sync (threads check and safety).
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn auto(
    prefix: &str,
    apiurl: &str,
    path: &str,
    url: &str,
    quality: &str,
    codex: &str,
    audioformat: &str,
    debug: bool,
    ttwatermark: bool,
    dublang: bool,
    fullaudio: bool,
    mute: bool,
) {
    println!("{prefix} getting stream URL for {}...", url);
    //inserting into HashMap
    let mut getstream_body = HashMap::new();
    getstream_body.insert("url", url);
    getstream_body.insert("vCodec", codec);
    getstream_body.insert("vQuality", quality);
    getstream_body.insert("aFormat", audioformat);
    let inttwm = &ttwatermark.to_string();
    let ifa = &fullaudio.to_string();
    let iam = &mute.to_string();
    let idl = &dublang.to_string();
    if ttwatermark == true {
        getstream_body.insert("isNotTTWatermark", inttwm);
    }
    if fullaudio == true {
        getstream_body.insert("isTTFullAudio", ifa);
    }
    if mute == true {
        getstream_body.insert("isAudioMuted", iam);
    }
    if dublang == true {
        getstream_body.insert("dubLang", idl);
    }
    //building the full url string
    let getstream_url = &format!("https://{apiurl}/api/json");

    if debug {
        println!(" ");
        println!("{prefix} {}", "====[ debug ]====");
        println!("{prefix} get stream url request url:");
        println!("{prefix} {}", getstream_url);
        println!("{prefix} get stream url request body:");
        println!(
            "{prefix} {}",
            serde_json::to_string(&getstream_body).unwrap()
        );
        println!("{prefix} {}", "====[ debug ]====");
        println!(" ");
    }

    getstream(prefix, &getstream_url, getstream_body, path);
}

pub fn audio(
    prefix: &str,
    apiurl: &str,
    path: &str,
    url: &str,
    quality: &str,
    codec: &str,
    audioformat: &str,
    ttwatermark: bool,
    debug: bool,
    dublang: bool,
    fullaudio: bool,
    mute: bool,
) {
    println!("{prefix} getting stream URL for {}...", url);
    //inserting into HashMap
    let mut getstream_body = HashMap::new();
    getstream_body.insert("isAudioOnly", "true");
    getstream_body.insert("url", url);
    getstream_body.insert("vCodec", codec);
    getstream_body.insert("vQuality", quality);
    getstream_body.insert("aFormat", audioformat);
    let inttwm = &ttwatermark.to_string();
    let ifa = &fullaudio.to_string();
    let iam = &mute.to_string();
    let idl = &dublang.to_string();
    if ttwatermark == true {
        getstream_body.insert("isNoTTWatermark", inttwm);
    }
    if fullaudio == true {
        getstream_body.insert("isTTFullAudio", ifa);
    }
    if mute == true {
        getstream_body.insert("isAudioMuted", iam);
    }
    if dublang == true {
        getstream_body.insert("dubLang", idl);
    }
    //making the full url needed
    let getstream_url = &format!("https://{apiurl}/api/json");

    if debug {
        println!(" ");
        println!("{prefix} {}", "====[ debug ]====");
        println!("{prefix} get stream url request url:");
        println!("{prefix} {}", getstream_url);
        println!("{prefix} get stream url request body:");
        println!(
            "{prefix} {}",
            serde_json::to_string(&getstream_body).unwrap()
        );
        println!("{prefix} {}", "====[ debug ]====");
        println!(" ");
    }

    getstream(prefix, &getstream_url, getstream_body, path);
}
