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
    let link_pattern = Regex::new(r"[https|http]*:\/\/[www\.]*aparat.com\/v\/(.....)$").unwrap();

    if let Some(captures) = link_pattern.captures(&args.link) {
        if let Some(group_1) = captures.get(1) {
            let hash_value = group_1.as_str();
            println!("{color_green}Video Hash value: {}{color_reset}", hash_value);

            let json_value = get_json_data(hash_value).unwrap();
            let links = &json_value["video"]["file_link_all"];
            let file_name = &json_value["video"]["title"].as_str().unwrap();

            match args.quality.as_str() {
                "MAX" => {
                    let last_obj = links.as_array().unwrap().last().unwrap();
                    let link = last_obj["urls"].as_array().unwrap().get(0).unwrap().as_str().unwrap();
                    println!("{color_green}Max quality: {}{color_reset}", last_obj["profile"]);
                    download_video(link, file_name, &args.save_folder);
                },
                _ => {
                    for link in links.as_array().unwrap() {
                        if link["profile"] == args.quality {
                            let link = link["urls"].as_array().unwrap().get(0).unwrap().as_str().unwrap();
                            download_video(link, file_name, &args.save_folder);
                            break;
                        }
                    }
                }
            }

        }
    }
}

fn get_json_data(hash_value: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let api_url = format!("https://www.aparat.com/etc/api/video/videohash/{}", hash_value);
    let res = reqwest::blocking::get(&api_url)?;

    let body = res.text()?;
    // Deserialize the JSON data into a dynamic Value
    let json_value: Value = serde_json::from_str(&body)?;

    Ok(json_value)
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