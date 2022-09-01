CREATE TABLE players (
    `id` VARCHAR(36),
    `name` VARCHAR(16) NOT NULL,
    PRIMARY KEY(`id`),
    UNIQUE KEY(`name`)
);