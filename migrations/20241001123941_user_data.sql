CREATE TABLE user_data (
    id          SERIAL PRIMARY KEY,
    user_id     VARCHAR(50) NOT NULL,
    user_name   VARCHAR(50) NOT NULL,
    user_mail   VARCHAR(50) NOT NULL,
    user_pass   TEXT NOT NULL
)
