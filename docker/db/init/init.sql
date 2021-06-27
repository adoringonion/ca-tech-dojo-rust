DROP TABLE IF EXISTS user;

CREATE TABLE user
(
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    token VARCHAR(255) NOT NULL
);

INSERT INTO user (name, token) VALUES ("taro", "aaaaa");


DROP TABLE IF EXISTS game_character;

CREATE TABLE game_character
(
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

INSERT INTO game_character (name) VALUES ("Oeeee");


DROP TABLE IF EXISTS user_character;

CREATE TABLE user_character
(
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    user_id INTEGER NOT NULL,
    character_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL
);

INSERT INTO user_character (user_id, character_id, quantity) VALUES (1, 1, 3);