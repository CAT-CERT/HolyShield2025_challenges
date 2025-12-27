CREATE DATABASE IF NOT EXISTS devilList;

USE devilList;

CREATE TABLE IF NOT EXISTS users (
    idx INT AUTO_INCREMENT PRIMARY KEY,
    id VARCHAR(50) NOT NULL UNIQUE,
    pw VARCHAR(50) NOT NULL,
    role VARCHAR(20) NOT NULL
);

INSERT INTO users (id, pw, role) VALUES ('admin', 'fake_pw', 'INQUISITOR');