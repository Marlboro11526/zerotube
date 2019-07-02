CREATE TABLE confirmation_emails (
    id VARCHAR(63) NOT NULL PRIMARY KEY,
    user_id VARCHAR(63) NOT NULL,
    expiry_date_time TIMESTAMP NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);
