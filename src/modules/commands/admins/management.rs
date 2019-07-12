use serenity::framework::standard::{
    macros::{command, group, check}, Args, CommandResult, CommandOptions,
    CheckResult, CommandError, Delimiter};

use serenity::model::prelude::Message;
use crate::core::consts::DB as db;
use serenity::prelude::Context;
use serenity::utils::parse_channel;
use serenity::model::id::ChannelId;

group!({
    name: "management",
    commands: [],
    sub_groups: [log],
    options: {
        only_in: "guilds",
//        checks: [Admin],
        required_permissions: [Administrator]
    },
});

group!({
    name: "log",
    options: {
        prefix: "log",
    },
    commands: [log_enable, log_disable, log_ignore, log_switch],
    sub_groups: [log_categories],
});

group!({
    name: "log_categories",
    options: {
        prefix: "category",
    },
    commands: [log_cat_enable, log_cat_disable, log_cat_reset],
});

#[command("enable")]
#[num_args(1)]
#[checks(Channel)]
pub fn log_enable(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mut guild_data = db.get_guild(msg.guild_id.unwrap().0 as i64).unwrap();
    if guild_data.log {
        return Err(CommandError("Logging is already enabled".to_string()));
    }
    let channel = args.parse::<ChannelId>().unwrap();

    guild_data.log_channel = channel.0 as i64;
    guild_data.log = true;
    db.update_guild(msg.guild_id.unwrap().0 as i64, guild_data).ok();


    msg.react(&ctx, '👍').ok();
    Ok(())
}

#[command("disable")]
pub fn log_disable(ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
    let mut guild_data = db.get_guild(msg.guild_id.unwrap().0 as i64).unwrap();
    if !guild_data.log {
        return Err(CommandError("Logging is already disabled".to_string()));
    }
    guild_data.log = false;
    guild_data.log_channel = 0;
    db.update_guild(msg.guild_id.unwrap().0 as i64, guild_data).unwrap();
    msg.react(&ctx, '👍').ok();

    Ok(())
}

#[command("switch")]
#[num_args(1)]
#[checks(Channel)]
pub fn log_switch(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mut guild_data = db.get_guild(msg.guild_id.unwrap().0 as i64).unwrap();
    if !guild_data.log {
        return Err(CommandError("Logging is disabled. Please enable logging first.".to_string()));
    }
    let channel = args.parse::<ChannelId>().unwrap();

    guild_data.log_channel = channel.0 as i64;
    db.update_guild(msg.guild_id.unwrap().0 as i64, guild_data).ok();


    msg.react(&ctx, '👍').ok();
    Ok(())
}

#[command("ignore")]
pub fn log_ignore(_ctx: &mut Context, _msg: &Message, _args: Args) -> CommandResult {
    Ok(())
}

#[command("enable")]
pub fn log_cat_enable(_ctx: &mut Context, _msg: &Message, _args: Args) -> CommandResult {
    Ok(())
}

#[command("disable")]
pub fn log_cat_disable(_ctx: &mut Context, _msg: &Message, _args: Args) -> CommandResult {
    Ok(())
}

#[command("reset")]
pub fn log_cat_reset(_ctx: &mut Context, _msg: &Message, _args: Args) -> CommandResult {
    Ok(())
}

//
//#[check]
//#[name = "Admin"]
//fn is_admin(ctx: &mut Context, msg: &Message, args: &mut Args, opts: &CommandOptions) -> CheckResult {
//    msg.member.unwrap();
//    CheckResult::Success
//}


#[check]
#[name = "Channel"]
fn is_valid_channel(ctx: &mut Context, msg: &Message, args: &mut Args, _: &CommandOptions) -> CheckResult {
//    let channel =
    let arg = args.current().unwrap();
    let channel = match parse_channel(arg) {
            Some(id) => ChannelId(id),
            None => match arg.parse::<ChannelId>() {
                Ok(id) => id,
                Err(_) => return CheckResult::new_user("Invalid Channel ID".to_string()),
            }
    };


    match channel.to_channel(&ctx).unwrap().guild() {
        Some(guild) => {
            if !(guild.read().guild_id == msg.guild_id.unwrap()) {
                return CheckResult::new_user("The log channel must be in the same guild.".to_string());
            }
        },
        None => return CheckResult::new_user("The log channel must be a guild channel, it cannot be private.".to_string()),
    }

    *args = Args::new(format!("{}", channel).as_str(), &[Delimiter::Single(' ')]);

    CheckResult::Success

}
