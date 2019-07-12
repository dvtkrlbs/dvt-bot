use super::schema::*;

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[primary_key(id)]
pub struct Guild {
    pub id: i64,
    pub log: bool,
    pub log_channel: i64,
    pub modlog: bool,
    pub modlog_channel: i64,
    pub autorole: bool,
    pub autoroles: Vec<i64>,
    pub prefix: String,
    pub logging: Vec<String>,
    pub disabled_commands: Vec<String>,
    pub ignored_channels: Vec<i64>,
    pub mod_roles: Vec<i64>,
    pub admin_roles: Vec<i64>
}

#[derive(Insertable)]
#[table_name = "guilds"]
pub struct NewGuild {
    pub id: i64,
}
