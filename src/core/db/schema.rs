table! {
    guilds (id) {
        id -> Int8,
        log -> Bool,
        log_channel -> Int8,
        modlog -> Bool,
        modlog_channel -> Int8,
        autorole -> Bool,
        autoroles -> Array<Int8>,
        prefix -> Text,
        logging -> Array<Text>,
        disabled_commands -> Array<Text>,
        ignored_channels -> Array<Int8>,
    }
}
