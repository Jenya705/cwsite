CREATE TABLE players (
    `id` VARCHAR(36),
    `name` VARCHAR(16) NOT NULL,
    `discord_id` BIGINT UNSIGNED NOT NULL,
    `role` BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY(`id`),
    UNIQUE KEY(`name`),
    UNIQUE KEY(`discord_id`),
    KEY(`role`)
);