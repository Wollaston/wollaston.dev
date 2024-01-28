CREATE TABLE IF NOT EXISTS projects (
    id          INTEGER PRIMARY KEY NOT NULL,
    title       VARCHAR(250)        NOT NULL,
    path        VARCHAR(250)        NOT NULL,
    github_link VARCHAR(250)        NOT NULL,
    description VARCHAR(250)        NOT NULL
);
