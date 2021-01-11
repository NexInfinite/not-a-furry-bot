 pub mod echo_mod {
     use serenity::{
         prelude::*,
         model::prelude::*,
         framework::standard::{
             CommandResult, macros::command, Args
         },
     };
     #[command]
     #[usage("<text>")]
     #[example("'hello i am echoing things'")]
     #[max_args(1)]
     #[min_args(1)]
     pub fn echo(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
         let text = args.current().unwrap().replace("'", "").replace('"', "");
         msg.channel_id.say(&ctx.http, format!("{}", text))?;
         return Ok(())
    }
}