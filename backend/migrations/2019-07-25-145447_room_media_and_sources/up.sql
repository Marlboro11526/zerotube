CREATE TABLE sources (
    name VARCHAR(63) NOT NULL PRIMARY KEY
);

INSERT INTO sources (name)
VALUES ('youtube');

CREATE TABLE room_media (
    id VARCHAR(63) NOT NULL PRIMARY KEY,
    room_id VARCHAR(63) NOT NULL,
    source VARCHAR(63) NOT NULL,
    name VARCHAR(127) NOT NULL,
    seconds INT NOT NULL,
    url VARCHAR(127) NOT NULL,
    FOREIGN KEY(room_id) REFERENCES rooms(id),
    FOREIGN KEY(source) REFERENCES sources(name)
);
