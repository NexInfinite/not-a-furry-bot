pub mod pfp_mod {
    use serenity::client::Context;
    use std::fs;
    use serenity::utils;
    use serde_json::Value;
    use std::io;
    use std::fs::File;
    use std::io::{Write};

    pub fn update_pfp(context: Context) {
        // update bot pfp OwO
        let changed = fs::read_to_string("loops/changed.txt").expect("Something went wrong reading the file");
        if changed == "false"{
            let mut found_valid_pfp = false;
            while found_valid_pfp == false {
                let get_pfp_image: fn() -> (String, Value) = || {
                    let body = reqwest::blocking::get("https://redpanda.pics/random").unwrap().text().unwrap();
                    let v: Value = serde_json::from_str(&body).unwrap();
                    let image_type = v["type"].to_string().replace('"', "");
                    return (image_type, v);
                };

                let (image_type, v) = get_pfp_image();
                let valid_file_types = vec!["png", "jpg", "jpeg"];
                if valid_file_types.iter().any(|&i| i==image_type) {
                    found_valid_pfp = true;
                    let image_url = v["url"].to_string().replace('"', "");
                    let response = reqwest::blocking::get(&image_url).unwrap();
                    let bytes = response.bytes().unwrap();

                    let mut slice: &[u8] = bytes.as_ref();
                    let mut out = File::create("image.jpg").expect("failed to create file");
                    io::copy(&mut slice, &mut out).expect("failed to copy content");

                    let base64 = utils::read_image("./image.jpg").expect("Failed to read image");
                    let _ = context.cache.write().user.edit(&context, |p|
                        p.avatar(Some(&base64)));

                    let mut file = File::create("handlers/changed.txt").unwrap();
                    let _ = file.write_all(b"true");

                    println!("Updated bot pfp!");
                }
                };
            }
    }

    pub fn reset_pfp_cycle() {
        let mut file = File::create("loops/changed.txt").unwrap();
        let _ = file.write_all(b"false");
    }
}