-- Your SQL goes here
-- Calendars: Persian, Gregorian, Chineese, Arabic,...
-- Item Kind: Goal, Note, Event, ...
-- Item Period Type? Weekly/Yearly/Seasonal/Monthly
--     General items are ordinary weekly item.
--     Objective items can by assigned to a year, a season, or a month. ex:
--         Persian Year 1403
--         Gregorian Season Spring of 2020
--         Arabic First Month of 1444

CREATE TABLE if not exists items (
    id                  INTEGER PRIMARY KEY NOT NULL,
    calendar            INTEGER NOT NULL,
    year                INTEGER,
    season              INTEGER,
    month               INTEGER,
    day                 INTEGER NOT NULL,
    kind                INTEGER NOT NULL,
    fixed_date          BOOL NOT NULL,
    all_day             BOOL NOT NULL,
    title               TEXT,
    note                TEXT,
    datetime            TEXT,
    duration            INTEGER,
    status              INTEGER,
    order_in_week       TEXT,
    order_in_resolution TEXT,
    sync                INTEGER,
    uuid                TEXT
);
