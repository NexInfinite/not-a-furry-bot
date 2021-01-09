pub mod rrole_mod {
    use serenity::{
        prelude::*,
        model::prelude::*,
        framework::standard::{
            CommandResult, macros::command, Args
        },
    };
    #[command]
    #[description("Remove a role from yourself. Usage !rrole <role_name>")]
    #[min_args(1)]
    #[max_args(1)]
    fn rrole(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
        let mut member = ctx.http.get_member(u64::from(msg.guild_id.unwrap()), u64::from(msg.author.id))?;
        let guild = &ctx.http.get_guild(u64::from(msg.guild_id.unwrap())).unwrap();
        let role_name = args.current().unwrap().replace('"', "");
        let role = guild.role_by_name(&role_name);
        &member.remove_role(&ctx.http, u64::from(role.unwrap().id));
        let _ = msg.channel_id.say(&ctx.http, format!("Role **{}** has been removed from **{}**", role_name, member.mention()));
        Ok(())
    }
}