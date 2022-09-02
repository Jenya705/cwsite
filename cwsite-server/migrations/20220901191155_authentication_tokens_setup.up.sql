CREATE TABLE authentication_tokens (
    `discord_id` BIGINT UNSIGNED NOT NULL,
    `token` VARCHAR(128) NOT NULL,
    KEY (`token`),
    FOREIGN KEY (`discord_id`) REFERENCES `players`(`discord_id`)
);