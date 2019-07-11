#![feature(result_map_or_else)]
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};

use serenity::model::prelude::{Message, UserId};
use serenity::prelude::Context;
use serenity::utils::parse_username;

group!({
    name: "general",
    commands: [avatar, ping],
});

#[command]
pub fn avatar(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let reply = args.current().map_or_else(
        || msg.author.face(),
        |mention| match parse_username(mention) {
            Some(id) => UserId(id).to_user(&ctx).unwrap().face(),
            None => match args.parse::<UserId>() {
                Ok(id) => id
                    .to_user(&ctx)
                    .map_or_else(|_| "Invalid user ID".to_string(), |user| user.face()),
                Err(_) => "Expected first argument as id or a mention!".to_string(),
            },
        },
    );

    msg.reply(ctx, reply.as_str())?;

    Ok(())
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
