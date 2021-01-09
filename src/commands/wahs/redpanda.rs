pub mod redpanda_mod {
    use serenity::{
        prelude::*,
        model::prelude::*,
        framework::standard::{
            CommandResult, macros::command
        },
        utils::Colour,
    };
    use serde_json::{
        Value,
    };

    #[command]
    #[description("Get a cutie red panda uwu")]
    #[max_args(0)]
    fn redpanda(ctx: &mut Context, msg: &Message) -> CommandResult {
        let body = reqwest::blocking::get("https://redpanda.pics/random")?.text()?;
        let v: Value = serde_json::from_str(&body)?;
        let image_url = v["url"].to_string().replace('"', "");
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name("Hey cutie! Have this red pandie");
                    a.url(&image_url);
                    a
                });
                e.image(format!("{}", &image_url));
                e.color(Colour::from_rgb(229, 106, 179));
                e.footer(|f| {
                    f.text("Made with love from https://redpanda.pics x Nexin")
                });
                e
            })
        })?;
        Ok(())
    }
}