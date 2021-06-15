DROP TABLE IF EXISTS user;

CREATE TABLE user
(
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    token VARCHAR(255) NOT NULL
);


INSERT INTO user (name, token) VALUES ("taro", "aaaaa");