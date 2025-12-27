CREATE TABLE users(
    `id` INT NOT NULL AUTO_INCREMENT,
    `user_id` VARCHAR(64) NOT NULL,
    `password` VARCHAR(255) NOT NULL,
    `email` VARCHAR(64) NOT NULL,
    `name` VARCHAR(32) NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    `role` INT NOT NULL,
    `uuid` VARCHAR(36) NOT NULL UNIQUE,
    PRIMARY KEY (`id`),
    UNIQUE KEY `users_email_uk` (`email`),
    UNIQUE KEY `users_user_id_uk` (`user_id`)
) DEFAULT CHARSET=utf8mb4;

CREATE TABLE board(
    `seq` INT NOT NULL AUTO_INCREMENT,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `title` VARCHAR(32) NOT NULL,
    `content` TEXT,
    `file_name` VARCHAR(255),
    `file_uploader_uuid` VARCHAR(36),
    PRIMARY KEY(`seq`)
) DEFAULT CHARSET=utf8mb4;
