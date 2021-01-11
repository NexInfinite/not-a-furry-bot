mod commands;
mod handlers;
mod loops;

use crate::handlers::Handler;

use crate::commands::{
    ECHO_COMMAND, EMBED_COMMAND, REDPANDA_COMMAND, ROLE_COMMAND, RROLE_COMMAND, BOOP_COMMAND, MY_HELP
};

use serenity::Client;
use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::{group};
use serenity::framework::standard::DispatchError::{NotEnoughArguments, TooManyArguments};
use std::fs;


#[group("misc")]
#[description("Miscellaneous commands that may be removed in the future.")]
#[commands(echo, embed)]
struct Misc;

#[group("admin")]
#[description("Commands used by either admins or people with specific roles.")]
#[commands(role, rrole)]
struct Admin;

#[group("wahs")]
#[description("Cute wahs to brighten up your day!")]
#[commands(redpanda, boop)]
struct Wahs;

fn main(){
    // std::thread::spawn(main_loop);
    let token = fs::read_to_string("token.txt").expect("Something went wrong reading the file");
    let mut client = Client::new(&token, Handler).unwrap();
    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .prefixes(vec!["f!", "nexin ", "nex ", "uwu ", "owo ", "cutie "])
            .no_dm_prefix(true)
            .on_mention(Some(serenity::model::id::UserId::from(797194753266810890)))
        )
        .group(&ADMIN_GROUP)
        .group(&MISC_GROUP)
        .group(&WAHS_GROUP)
        .help(&MY_HELP)
    .on_dispatch_error(|context, msg, error| {
        match error {
            NotEnoughArguments { min, given } => {
                let s = format!("Not enough arguments give! {} are needed, {} where given", min, given);
                let _ = msg.channel_id.say(&context.http, &s);
            },
            TooManyArguments { max, given } => {
                let s = format!("Too many arguments given! {} max, {} where given. Try wrapping them in \"quotes\". As of now only `\"` quotes work.", max, given);
                let _ = msg.channel_id.say(&context.http, &s);
            },
            _ => println!("Unhandled dispatch error."),
        }
    })
    );
    println!("Logging in...");
    client.start().expect("Could not start client.");
}