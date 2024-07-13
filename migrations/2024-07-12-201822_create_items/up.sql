-- Your SQL goes here
-- Calendars: Persian, Gregorian, Chineese, Arabic,...
-- Item Kind: Goal, Note, Event
-- Item is resolution?
--     a resolution can by assigned to a year, a season, or a month, or a special period!
--     Item without a resolution is ordinary weekly item. ex:
--     Persian Year 1403
--     Gregorian Season Spring of 2020
--     Arabic First Month of 1444
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
