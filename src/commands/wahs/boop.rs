pub mod boop_mod {
    use serenity::{
        prelude::*,
        model::prelude::*,
        framework::standard::{
            CommandResult, macros::command, Args
        },
        utils::Colour,
    };
    use rand::prelude::SliceRandom;

    #[command]
    #[description("Boop someone.")]
    #[usage("<user>")]
    #[example("@Nexin 🌸#0001")]
    #[max_args(1)]
    #[min_args(1)]
    fn boop(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
        let boops = vec!["https://media1.tenor.com/images/3aa7d205414420870d9050eaf4b1c1df/tenor.gif",
                         "https://media1.tenor.com/images/d4c62585900b8bd22857d320c13cb080/tenor.gif",
                         "https://media.giphy.com/media/TjLx5TCfTWiE0dSZwS/giphy.gif",
                         "https://media.giphy.com/media/KayxhQDpUIZYEuagzX/giphy.gif"];
        let mut rng = rand::thread_rng();
        let boop =  boops.choose(&mut rng).unwrap();
        let boopee_id: u64 = args.current().unwrap().replace("<@!", "").replace(">", "").to_string().parse().unwrap();
        let boopee = ctx.http.get_user(boopee_id)?;
        if boopee.to_owned().id == msg.author.id {
            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(format!("Nexin booped {}, Kayooooot!", msg.author.name));
                    e.author(|a| {
                        a.name("You can't boop yourself, but I can boop you!");
                        a
                    });
                    e.image(format!("{}", &boop));
                    e.color(Colour::from_rgb(229, 106, 179));
                    e.footer(|f| {
                        f.text("Made with love from Nexin")
                    });
                    e
                })
            })?;
        } else {
            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(format!("{} booped {}, Kayooooot!", msg.author.name, boopee.to_owned().name));
                    e.image(format!("{}", &boop));
                    e.color(Colour::from_rgb(229, 106, 179));
                    e.footer(|f| {
                        f.text("Made with love from Nexin")
                    });
                    e
                })
            })?;
        }
        Ok(())
    }
}