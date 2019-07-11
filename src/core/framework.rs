use crate::core::consts::DB as db;
use crate::core::db::models::Guild;
use crate::macros;
use crate::modules::commands::general::*;
use serenity::framework::standard::DispatchError;
use serenity::framework::StandardFramework;
use serenity::model::id::GuildId;

pub struct DvtFramework {}

impl DvtFramework {
    pub fn new() -> StandardFramework {
        StandardFramework::new()
            .configure(|c| {
                c.allow_dm(true)
                    //                .on_mention(true)
                    .ignore_bots(true)
                    .case_insensitivity(true)
                    .prefix("d!")
                    .dynamic_prefix(|_, msg| {
                        if msg.is_private() {
                            return Some(String::new());
                        } else {
                            let guild_id = msg.guild_id.unwrap_or(GuildId(0));
                            dbg!(guild_id);
                            if let Ok(settings) = db.get_guild(guild_id.0 as i64) {
                                dbg!(&settings.prefix);
                                return Some(settings.prefix);
                            }
                        }
                        None
                    })
            })
            .before(|ctx, message, command_name| {
                if let false = message.is_private() {
                    let guild_id = message.guild_id.unwrap_or(GuildId(0));
                    if let Ok(guild_data) = db.get_guild(guild_id.0 as i64) {
                        if guild_data
                            .ignored_channels
                            .contains(&(message.channel_id.0 as i64))
                        {
                            dbg!("channel is ignored");
                            // TODO add some mechanism to allow mods and admins to bypass this check
                            return false;
                        }
                        if guild_data
                            .disabled_commands
                            .contains(&command_name.to_string())
                        {
                            dbg!("command is disabled");
                            return false;
                        }
                    }
                }
                true
            })
            .on_dispatch_error(|ctx, message, error| {
                use serenity::framework::standard::DispatchError;
                match error {
                    DispatchError::LackingPermissions(perm) => {
                        message.channel_id.say(
                            ctx,
                            format!(
                        "You lack the following permissions needed to execute this command: {:?}",
                        perm
                    ),
                        );
                    }
                    DispatchError::Ratelimited(time) => {
                        message.channel_id.say(
                            ctx,
                            format!(
                            "A bit too soon for that. Please wait {} seconds before trying again."
                            ,time),
                        );
                    }
                    DispatchError::NotEnoughArguments { min, given } => {
                        message.channel_id.say(
                            ctx,
                            format!(
                                "Too few arguments provided. {} given, {} minimum.",
                                given, min
                            ),
                        );
                    }
                    DispatchError::TooManyArguments { max, given } => {
                        message.channel_id.say(
                            ctx,
                            format!(
                                "Too many arguments provided. {} given, {} maximum.",
                                given, max
                            ),
                        );
                    }
                    DispatchError::OnlyForDM => {
                        message
                            .channel_id
                            .say(ctx, "This command is only available in private channels.");
                    }
                    DispatchError::OnlyForGuilds => {
                        message
                            .channel_id
                            .say(ctx, "This command is only available in guilds.");
                    }
                    _ => (),
                }
            })
            .group(&GENERAL_GROUP)
    }
}
