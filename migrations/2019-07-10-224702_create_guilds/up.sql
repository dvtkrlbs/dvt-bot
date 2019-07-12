CREATE TABLE guilds (
                        id BIGINT PRIMARY KEY,
                        log BOOL NOT NULL DEFAULT 'f',
                        log_channel BIGINT NOT NULL DEFAULT 0,
                        modlog BOOL NOT NULL DEFAULT 'f',
                        modlog_channel BIGINT NOT NULL DEFAULT 0,
                        autorole BOOL NOT NULL DEFAULT 'f',
                        autoroles BIGINT [] NOT NULL DEFAULT array[]::bigint[],
                        prefix TEXT NOT NULL DEFAULT 'm!',
                        logging TEXT [] NOT NULL DEFAULT array[]::text[],
                        disabled_commands TEXT [] NOT NULL DEFAULT array[]::text[],
                        ignored_channels BIGINT [] NOT NULL DEFAULT array[]::bigint[],
                        mod_roles BIGINT [] NOT NULL DEFAULT array[]::bigint[],
                        admin_roles BIGINT [] NOT NULL DEFAULT array[]::bigint[]
)