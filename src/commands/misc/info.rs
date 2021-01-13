pub mod info_mod {
    use serenity::{
        prelude::*,
        model::prelude::*,
        framework::standard::{
            CommandResult, macros::command
        },
        utils::Colour,
    };

    #[command]
    #[description("Get information about the bot and its owner")]
    fn info(ctx: &mut Context, msg: &Message) -> CommandResult {
        let colour = Colour::from_rgb(229, 106, 179);
        msg.channel_id.send_message(&ctx.http, |f| {
            f.embed(|e| e.title(format!("About the bot"))
                .description(format!("This bot was made by [nexin](https://github.com/nexinfinite/not-a-furry-bot) to help learn rust. \
                This bot started out as just a normal bot with commands like `echo` but quickly advanced to use more api's including [redpanda.pics](https://redpanda.pics) \
                which was also made by nexin. I hope you enjoy. [Click here for more](https://www.youtube.com/watch?v=DLzxrzFCyOs)"))
                .color(colour)
            )
        })?;

        Ok(())
    }
}