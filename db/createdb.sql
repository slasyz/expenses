CREATE TABLE expenses (
    id     bigserial PRIMARY KEY,
    title  text      NOT NULL,
    date   date      NOT NULL,
    amount bigint    NOT NULL
);
