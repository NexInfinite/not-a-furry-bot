pub mod help_mod {
    use serenity::framework::standard::{
        help_commands,
        Args,
        HelpOptions,
        CommandGroup,
        CommandResult,
    };
    use serenity::framework::standard::macros::help;
    use serenity::model::prelude::{Message, UserId};
    use serenity::client::{Context};
    use std::collections::HashSet;

    #[help]
    #[embed_success_colour(fooyoo)]
    fn my_help(
        context: &mut Context,
        msg: &Message,
        args: Args,
        help_options: &'static HelpOptions,
        groups: &[&'static CommandGroup],
        owners: HashSet<UserId>
    ) -> CommandResult {
        help_commands::with_embeds(context, msg, args, help_options, groups, owners)
    }
}