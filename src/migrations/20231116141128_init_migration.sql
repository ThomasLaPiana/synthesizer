----------------------------------------------
-- Create the Pipelines table and seed data --
----------------------------------------------
CREATE TABLE IF NOT EXISTS pipelines (
    id TEXT NOT NULL PRIMARY KEY,
    schedule TEXT NOT NULL
);
-- Seed Pipelines data
INSERT INTO pipelines
VALUES(
        'pipeline1',
        '1 * * * *'
    ),
    (
        'pipeline2',
        '1 * * * *'
    );

--------------------------------------
-- Create Tasks table and seed data --
--------------------------------------
CREATE TABLE IF NOT EXISTS tasks (
    id TEXT NOT NULL PRIMARY KEY,
    pipeline_id TEXT NOT NULL,
    command TEXT NOT NULL
);
-- Seed Tasks data
INSERT INTO tasks
VALUES(
        'task1',
        'pipeline1',
        'echo yes'
    ),
    (
        'task2',
        'pipeline2',
        'echo yes'
    );
