-- Your SQL goes here
CREATE TABLE persons (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    forename VARCHAR NOT NULL,
    age INTEGER NOT NULL
);

INSERT INTO persons (name, forename, age) VALUES ("von Riva", "Geralt", 100);
INSERT INTO persons (name, forename, age) VALUES ("Merigold", "Triss", 50);
INSERT INTO persons (name, forename, age) VALUES ("von Vengerberg", "Yennefer", 95);
INSERT INTO persons (name, forename, age) VALUES ("von Kaer Morhen", "Vesemir", 250);
INSERT INTO persons (name, forename, age) VALUES ("von Vengerberg", "Ciri", 25);
