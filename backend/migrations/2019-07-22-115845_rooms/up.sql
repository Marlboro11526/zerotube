CREATE TABLE rooms (
    id VARCHAR(63) NOT NULL PRIMARY KEY,
    description VARCHAR(511) NOT NULL,
    name VARCHAR(63) NOT NULL UNIQUE,
    public TINYINT(1) NOT NULL,
    url VARCHAR(63) NOT NULL UNIQUE
);