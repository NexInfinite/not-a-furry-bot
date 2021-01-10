pub mod event_handler_mod {
    use serenity::prelude::*;
    use serenity::model::prelude::*;
    use chrono::{Timelike, Utc};
    use std::fs;
    use serenity::utils;
    use serde_json::Value;
    use std::io;
    use std::fs::File;
    use std::io::{Write};

    pub struct Handler;
    impl EventHandler for Handler {
        fn message(&self, context: Context, msg: Message) {
            // Message logging in console and ignoring self messages
            if msg.author.id == 797194753266810890{
                return println!("<message from bot> {}", msg.content);
            } else {
                println!("<message from {}> {}", msg.author.name, msg.content);
            }
            // Trigger commands

            // Hello responses
            let hello_triggers = vec!["hello", "hi", "hiya", "helo", "heya", "hey", "hai"];
            for &trigger in &hello_triggers {
                if msg.content.to_lowercase().starts_with(trigger) {
                    let response = random_response(vec!["Hey!", "Hey, hows it going?", "Hello!", "Hiya.", "Whats up?", "Wassup?", "Hm? Oh, hey!", "Hellooooooo"]);
                    let _ = msg.channel_id.say(&context, format!("{}", response));
                }
            }

            // Goodbye responses
            let hello_triggers = vec!["cya", "gtg", "bye", "byee", "goodbye"];
            for &trigger in &hello_triggers {
                if msg.content.to_lowercase().starts_with(trigger) {
                    let response = random_response(vec!["Cya!", "Byeee!", "I'll be waiting", "See ya soon!", "Goodbye!"]);
                    let _ = msg.channel_id.say(&context, format!("{}", response));
                }
            }

            // bruh
            let bruh_triggers = vec!["bruh"];
            for &trigger in &bruh_triggers {
                if msg.content.to_lowercase().starts_with(trigger) {
                    let response = random_response(vec!["bruh moment", "momentum of the bruh", "bruh sound effect #12", "https://cdn.discordapp.com/emojis/655591384467898398.gif?v=1"]);
                    let _ = msg.channel_id.say(&context, format!("{}", response));
                }
            }

            // Edit the bot?! OwO
            let changed = fs::read_to_string("handlers/changed.txt").expect("Something went wrong reading the file");
            let now = Utc::now();
            let minute = now.minute();
            if minute == 00 && changed == "false"{
                let body = reqwest::blocking::get("https://redpanda.pics/random").unwrap().text().unwrap();
                let v: Value = serde_json::from_str(&body).unwrap();
                let image_type = v["type"].to_string().replace('"', "");

                let valid_file_types = vec!["png", "jpg", "jpeg"];
                if valid_file_types.iter().any(|&i| i==image_type) {
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
            } else if minute == 01 && changed == "true" {
                let mut file = File::create("handlers/changed.txt").unwrap();
                let _ = file.write_all(b"false");
            }
        }
        fn ready(&self, _context: Context, data: Ready){
            println!("Logged in as {}!", data.user.name);
        }
    }

    fn random_response(responses: Vec<&'static str>) -> &'static str {
        use rand::prelude::SliceRandom;
        let mut rng = rand::thread_rng();
        return responses.choose(&mut rng).unwrap();
    }

}