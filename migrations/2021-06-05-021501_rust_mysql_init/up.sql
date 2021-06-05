-- Your SQL goes here

CREATE TABLE user
(
    id INT(10) NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(40) NOT NULL,
    token_id INT(10) NOT NULL
);

CREATE TABLE token
(
    id INT(10) NOT NULL AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(40) NOT NULL
);

INSERT INTO token (value) VALUES ("test");
INSERT INTO user (name, token_id) VALUES ("taro", 1);