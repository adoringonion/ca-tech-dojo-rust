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
    name VARCHAR(255) NOT NULL,
    rarity INTEGER
);

INSERT INTO game_character (name, rarity) VALUES ("t_chara01", 0);
INSERT INTO game_character (name, rarity) VALUES ("t_chara02", 0);
INSERT INTO game_character (name, rarity) VALUES ("t_chara03", 0);

INSERT INTO game_character (name, rarity) VALUES ("t_chara04", 1);
INSERT INTO game_character (name, rarity) VALUES ("t_chara05", 1);
INSERT INTO game_character (name, rarity) VALUES ("t_chara06", 1);

INSERT INTO game_character (name, rarity) VALUES ("t_chara07", 2);
INSERT INTO game_character (name, rarity) VALUES ("t_chara08", 2);
INSERT INTO game_character (name, rarity) VALUES ("t_chara09", 2);

INSERT INTO game_character (name, rarity) VALUES ("t_chara10", 3);
INSERT INTO game_character (name, rarity) VALUES ("t_chara11", 3);
INSERT INTO game_character (name, rarity) VALUES ("t_chara12", 3);


DROP TABLE IF EXISTS user_has_character;

CREATE TABLE user_has_character
(
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    user_id INTEGER NOT NULL,
    character_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    UNIQUE user_character_index (user_id, character_id)
);

INSERT INTO user_has_character (user_id, character_id, quantity) VALUES (1, 1, 3);