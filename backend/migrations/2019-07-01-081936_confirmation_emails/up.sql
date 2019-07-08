CREATE TABLE confirmation_emails (
    id VARCHAR(63) NOT NULL PRIMARY KEY,
    expiry_date_time TIMESTAMP NOT NULL,
    user_id VARCHAR(63) NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);
