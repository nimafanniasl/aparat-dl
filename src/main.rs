use std::fs::File;
use clap::Parser;
use regex::Regex;
use reqwest;
use serde_json::Value;
use inline_colorization::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    link: String,

    #[arg(short, long, default_value = "MAX")]
    quality: String,

    #[arg(short, long, default_value = ".")]
    save_folder: String,
}

fn main() {
    let args = Args::parse();
    download_vid_with_hash(&args.link, &args.save_folder, &args.quality)
}

fn download_vid_with_hash(link: &str, save_folder: &str, quality: &str) {
    let vid_link_pattern = Regex::new(r"[https|http]*://[www\.]*aparat.com/v/(.....)$").unwrap();
    let vid_hash_pattern = Regex::new(r"^(.{5})$").unwrap();
    let playlist_link_pattern = Regex::new(r"[https|http]*://[www\.]*aparat\.com/v/(.....)\?playlist=(\d*$)$").unwrap();

    if let Some(captures) = vid_link_pattern.captures(link) {
        let hash_value = captures.get(1).unwrap().as_str();
        println!("{color_green}Video Hash value: {}{color_reset}", hash_value);

        let json_value = get_json_data_vid(hash_value).unwrap();
        // let links = &json_value["video"]["file_link_all"];
        let file_name = &json_value["video"]["title"].as_str().unwrap();

        let dl_link = get_dl_url(&json_value, &quality);

        download_video(&dl_link, file_name, save_folder);

    }
    else if let Some(captures) = vid_hash_pattern.captures(link) {
        let hash_value = captures.get(1).unwrap().as_str();
        println!("{color_green}Video Hash value: {}{color_reset}", hash_value);

        let json_value = get_json_data_vid(hash_value).unwrap();
        // let links = &json_value["video"]["file_link_all"];
        let file_name = &json_value["video"]["title"].as_str().unwrap();

        let dl_link = get_dl_url(&json_value, &quality);

        download_video(&dl_link, file_name, save_folder);

    }
    else if let Some(captures) = playlist_link_pattern.captures(link) {
        let playlist_id = captures.get(2).unwrap().as_str();

        let api_url = format!("https://www.aparat.com/api/fa/v1/video/playlist/one/playlist_id/{}", playlist_id);
        let res = reqwest::blocking::get(&api_url).unwrap();

        let body = res.text().unwrap();
        // Deserialize the JSON data into a dynamic Value
        let json_value: Value = serde_json::from_str(&body).unwrap();
        let hashes = get_videos_playlist(&json_value);

        println!("Found {} Videos in the playlist: ", hashes.len());
        for hash in hashes {
            println!("Video Hash: {hash}");
            download_vid_with_hash(hash.as_str(), save_folder, quality);
        }

    }
}

fn get_dl_url(json_value: &Value, quality: &str) -> String {

    let links = &json_value["video"]["file_link_all"];
    let file_name = &json_value["video"]["title"].as_str().unwrap();

    match quality {
        "MAX" => {
            let last_obj = links.as_array().unwrap().last().unwrap();
            let link = last_obj["urls"].as_array().unwrap().get(0).unwrap().as_str().unwrap();
            println!("{color_green}Max quality: {}{color_reset}", last_obj["profile"]);
            return link.to_string();
        },
        _ => {
            for link in links.as_array().unwrap() {
                if link["profile"] == quality {
                    let link = link["urls"].as_array().unwrap().get(0).unwrap().as_str().unwrap();
                    return link.to_string();
                }
            }
            String::new()
        }

    }
}

fn get_json_data_vid(hash_value: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let api_url = format!("https://www.aparat.com/etc/api/video/videohash/{}", hash_value);
    let res = reqwest::blocking::get(&api_url)?;

    let body = res.text()?;
    // Deserialize the JSON data into a dynamic Value
    let json_value: Value = serde_json::from_str(&body).unwrap();

    Ok(json_value)
}

fn get_videos_playlist(json_value: &Value) -> Vec<String> {

    let mut videos_in_playlist: Vec<String> = Vec::new();

    let included = &json_value["included"].clone();

    let frame_vidhash_regex = Regex::new(r"https:\/\/www\.aparat\.com\/video\/video\/embed\/videohash\/(.....)\/vt\/frame").unwrap();

    for data in included.as_array().unwrap().iter() {
        if data["type"].as_str().unwrap() == "Video" {
            let frame_link = &data["attributes"]["frame"].as_str().unwrap();
            if let Some(captures) = frame_vidhash_regex.captures(&frame_link) {
                let vid_hash = String::from(captures.get(1).unwrap().as_str());
                videos_in_playlist.push(vid_hash);
            }
        }
    }

    return videos_in_playlist

}

fn download_video(url: &str, file_name: &str, save_path: &str) {
    let response = reqwest::blocking::get(url);

    match response {
        Ok(mut file) => {
            let mut output = File::create(format!("{save_path}/{file_name}.mp4")).expect("Failed to create file");
            std::io::copy(&mut file, &mut output).expect("Failed to copy content");
            println!("{color_green}Video downloaded successfully!{color_reset}");
        },
        Err(err) => {
            eprintln!("{color_red}Error downloading video: {:?}{color_reset}", err);
        }
    }
}