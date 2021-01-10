pub mod event_handler_mod {
    use serenity::prelude::*;
    use serenity::model::prelude::*;

    use crate::loops::main_mod::main_loop;

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
        }
        fn ready(&self, context: Context, data: Ready){
            println!("Logged in as {}!", data.user.name);
            std::thread::spawn(|| main_loop(context));
        }
    }

    fn random_response(responses: Vec<&'static str>) -> &'static str {
        use rand::prelude::SliceRandom;
        let mut rng = rand::thread_rng();
        return responses.choose(&mut rng).unwrap();
    }

}