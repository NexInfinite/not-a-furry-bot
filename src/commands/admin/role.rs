pub mod role_mod {
    use serenity::{
        prelude::*,
        model::prelude::*,
        framework::standard::{
            CommandResult, macros::command, Args
        },
    };
    #[command]
    #[description("Give yourself a role.")]
    #[usage("<role name>")]
    #[example("'this is a role'")]
    #[only_in(guilds)]
    #[required_permissions(MANAGE_ROLES)]
    #[min_args(1)]
    #[max_args(1)]
    fn role(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
        let mut member = ctx.http.get_member(u64::from(msg.guild_id.unwrap()), u64::from(msg.author.id))?;
        let guild = &ctx.http.get_guild(u64::from(msg.guild_id.unwrap())).unwrap();
        let role_name = args.current().unwrap().replace('"', "");
        let role = guild.role_by_name(&role_name);
        &member.add_role(&ctx.http, u64::from(role.unwrap().id));
        let _ = msg.channel_id.say(&ctx.http, format!("Role **{}** has been given to **{}**", role_name, member.mention()));
        Ok(())
    }
}