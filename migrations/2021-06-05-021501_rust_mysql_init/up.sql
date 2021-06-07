-- Your SQL goes here

CREATE TABLE user
(
    id INT(10) NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(40) NOT NULL,
    token VARCHAR(50) NOT NULL
);


INSERT INTO user (name, token) VALUES ("taro", "aaaaa");