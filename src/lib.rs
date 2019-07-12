#![feature(result_map_or_else)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

pub mod core;
pub mod macros;
pub mod modules;

//use serenity::framework::StandardFramework;
use serenity::prelude::{Context, EventHandler};
use serenity::model::{
    id::{
        ChannelId, MessageId, GuildId
    }, gateway::Ready, guild::Guild};
use crate::core::consts::DB as db;
use crate::core::consts::*;
use serenity::utils::Colour;

pub struct Handler {}

impl EventHandler for Handler {
    fn message_delete(&self, ctx: Context, cid: ChannelId, deleted_id: MessageId) {
        let (_channel_name, guild_id) = {
            let channel_lock = ctx.cache.read().guild_channel(&cid);
            if let Some(channel_lock) = channel_lock {
                let ch = channel_lock.read();
                (ch.name.clone(), ch.guild_id)
            } else {
                ("unknown".to_string(), GuildId(0))
            }
        };
        if let Ok(guild_data) = db.get_guild(guild_id.0 as i64) {
            if guild_data.logging.contains(&String::from("message_delete")) { return; }
            let log_channel = ChannelId(guild_data.log_channel as u64);
            if guild_data.log && guild_data.log_channel != 0 {
                if guild_data.ignored_channels.contains(&(cid.0 as i64)) { return; }
                if let Some(msg) = ctx.cache.read().message(&cid, &deleted_id) {
                    if msg.author.bot { return; }
                    log_channel.send_message(&ctx,|m| {
                        m.embed(|e| {
                            e.author(|a| {
                                a.name(msg.author.tag())
                                    .icon_url(msg.author.face())
                            })
                                .title(format!("Message sent by @{} deleted in @{}", msg.author.name, ctx.cache.read().guild_channel(cid).unwrap().read().name))
                                .colour(Colour::RED)
                                .description(&msg.content)
                                .footer(|f| {
                                    f.text(format!("Author: {} | Message ID: {}", msg.author.id, deleted_id))
                                })
                                .timestamp(deleted_id.created_at().to_rfc3339())
                        })
                    }).ok();
                } else {
                    log_channel.send_message(&ctx, |m| {
                        m.embed(|e| {
                            e.author(|a| {
                                a.name(&guild_id.to_guild_cached(&ctx).unwrap().read().name)
                                    .icon_url(guild_id.to_guild_cached(&ctx).unwrap().read().icon_url().unwrap_or_else( || "".to_string()))
                            })
                                .title(format!("Uncached Message sent in {} deleted", ctx.cache.read().guild_channel(cid).unwrap().read().name))
                                .colour(Colour::RED)
                                .footer(|f| {
                                    f.text(format!("Author: ? | Message ID: {}", deleted_id))
                                })
                                .timestamp(deleted_id.created_at().to_rfc3339())
                        })
                    }).ok();
                }
            }
        }
    }
    fn ready(&self, ctx: Context, _ready: Ready) {
        ctx.cache.write().settings_mut().max_messages(MESSAGE_CACHE);
    }

    fn guild_create(&self, _ctx: Context, guild: Guild, is_new: bool) {
        if is_new {
            db.new_guild(guild.id.0 as i64).unwrap();
        }
    }
}
