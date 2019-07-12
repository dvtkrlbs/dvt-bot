#![allow(clippy::new_ret_no_self)]
use crate::core::consts::DB as db;
//use crate::core::db::models::Guild;
use crate::modules::commands::general::*;
use crate::modules::commands::admins::management::*;
use serenity::prelude::{Context};
use serenity::framework::standard::{
    DispatchError, help_commands, macros::help, Args, CommandGroup, HelpOptions, CommandResult};
use serenity::framework::StandardFramework;
use serenity::model::{id::{GuildId, UserId}, channel::Message};
use std::collections::HashSet;

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
                            if let Ok(settings) = db.get_guild(guild_id.0 as i64) {
                                return Some(settings.prefix);
                            }
                        }
                        None
                    })
            })
            .before(|_ctx, message, command_name| {
                if let false = message.is_private() {
                    let guild_id = message.guild_id.unwrap_or(GuildId(0));
                    if let Ok(guild_data) = db.get_guild(guild_id.0 as i64) {
                        if guild_data
                            .ignored_channels
                            .contains(&(message.channel_id.0 as i64))
                        {
                            // TODO add some mechanism to allow mods and admins to bypass this check
                            return false;
                        }
                        if guild_data
                            .disabled_commands
                            .contains(&command_name.to_string())
                        {
                            return false;
                        }
                    }
                }
                true
            })
            .after(|ctx, msg, _cmd_name, result| {
                match result {
                    Ok(()) => (),
                    Err(e) => {
                        msg.channel_id.say(ctx, e.0).ok();
                    },
                };
            })
            .on_dispatch_error(|ctx, message, error| match error {
                DispatchError::LackingPermissions(perm) => {
                    message.channel_id.say(
                        ctx,
                        format!(
                        "You lack the following permissions needed to execute this command: {:?}",
                        perm
                    )).ok();
                }
                DispatchError::Ratelimited(time) => {
                    message.channel_id.say(
                        ctx,
                        format!(
                            "A bit too soon for that. Please wait {} seconds before trying again.",
                            time
                        )).ok();
                }
                DispatchError::NotEnoughArguments { min, given } => {
                    message.channel_id.say(
                        ctx,
                        format!(
                            "Too few arguments provided. {} given, {} minimum.",
                            given, min
                        )).ok();
                }
                DispatchError::TooManyArguments { max, given } => {
                    message.channel_id.say(
                        ctx,
                        format!(
                            "Too many arguments provided. {} given, {} maximum.",
                            given, max
                        )).ok();
                }
                DispatchError::OnlyForDM => {
                    message.channel_id.say(ctx, "This command is only available in private channels.").ok();
                }
                DispatchError::OnlyForGuilds => {
                    message.channel_id.say(ctx, "This command is only available in guilds.").ok();
                }
                _ => (),
            })
            .help(&MY_HELP)
            .group(&GENERAL_GROUP)
            .group(&MANAGEMENT_GROUP)
    }
}

#[help]
#[individual_command_tip =
"If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
// On another note, you can set up the help-menu-filter-behaviour.
// Here are all possible settings shown on all possible options.
// First case is if a user lacks permissions for a command, we can hide the command.
#[lacking_permissions = "Hide"]
// If the user is nothing but lacking a certain role, we just display it hence our variant is `Nothing`.
#[lacking_role = "Nothing"]
// The last `enum`-variant is `Strike`, which ~~strikes~~ a command.
#[wrong_channel = "Strike"]
// Serenity will automatically analyse and generate a hint/tip explaining the possible
// cases of ~~strikethrough-commands~~, but only if
// `strikethrough_commands_tip(Some(""))` keeps `Some()` wrapping an empty `String`, which is the default value.
// If the `String` is not empty, your given `String` will be used instead.
// If you pass in a `None`, no hint will be displayed at all.
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
