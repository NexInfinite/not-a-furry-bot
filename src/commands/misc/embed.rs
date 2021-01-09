pub mod embed_mod {
    use serenity::{
        prelude::*,
        model::prelude::*,
        framework::standard::{
            CommandResult, macros::command, Args
        },
        utils::Colour,
    };
    #[command]
    #[description("Usage: !embed <title> <descirption> [<r> <g> <b>]")]
    #[min_args(2)]
    #[max_args(5)]
    fn embed(ctx: &mut Context, msg: &Message, mut arguments: Args) -> CommandResult {
        let title = arguments.current().unwrap().replace('"', "");
        arguments.advance();
        let description = arguments.current().unwrap().replace('"', "");
        arguments.advance();
        let mut colour = Colour::from_rgb(229, 106, 179);
        let mut colour_error = false;
        if arguments.current() != None {
            for n in 1..3 {
                arguments.advance();
                if arguments.current() == None {
                    colour_error = true;
                    msg.channel_id.say(&ctx.http, format!("You've forgotten to add a colour rgb only, found **{}/3**! Defaulting to base colour of **pink**!", n))?;
                    break
                }
            }
            if colour_error == false {
                for _n in 1..3 {
                    arguments.rewind();
                }
                let red: u8 = arguments.current().unwrap().replace('"', "").to_string().parse().unwrap();
                arguments.advance();
                let green: u8 = arguments.current().unwrap().replace('"', "").to_string().parse().unwrap();
                arguments.advance();
                let blue: u8 = arguments.current().unwrap().replace('"', "").to_string().parse().unwrap();
                colour = Colour::from_rgb(red, green, blue);
            }
        }
        msg.channel_id.send_message(&ctx.http, |f| {
            f.embed(|e| e.title(format!("{}", title))
                .description(format!("{}", description))
                .color(colour)
            )
        })?;

        Ok(())
    }
}