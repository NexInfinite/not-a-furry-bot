 pub mod echo_mod {
     use serenity::{
         prelude::*,
         model::prelude::*,
         framework::standard::{
             CommandResult, macros::command,
         },
     };
     #[command]
     pub fn echo(ctx: &mut Context, msg: &Message) -> CommandResult {
        msg.channel_id.say(&ctx.http, format!("{}", msg.content.strip_prefix("!echo").unwrap()))?;
        return Ok(())
    }
}