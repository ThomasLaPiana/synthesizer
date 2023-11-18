----------------------------------------------
-- Create the Pipelines table and seed data --
----------------------------------------------
CREATE TABLE IF NOT EXISTS pipelines (
    id TEXT NOT NULL UNIQUE,
    name TEXT,
    schedule TEXT NOT NULL
);
-- Seed Pipelines data
INSERT INTO pipelines
VALUES(
        'pipeline1',
        'Pipeline 1',
        '1 * * * *'
    ),
    (
        'pipeline2',
        'Pipeline 2',
        '1 * * * *'
    );

--------------------------------------
-- Create Tasks table and seed data --
--------------------------------------
CREATE TABLE IF NOT EXISTS tasks (
    pipeline_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE,
    command TEXT NOT NULL
);
-- Seed Tasks data
INSERT INTO tasks
VALUES(
        'pipeline1',
        'task1',
        'echo yes'
    ),
    (
        'pipeline2',
        'task2',
        'echo yes'
    );
