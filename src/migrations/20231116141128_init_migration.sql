----------------------------------------------
-- Create the Pipelines table and seed data --
----------------------------------------------
CREATE TABLE IF NOT EXISTS pipelines (
    id TEXT NOT NULL PRIMARY KEY,
    schedule TEXT NOT NULL
    -- TODO: Add created_at
    -- TODO: Add modified_at
    -- TODO: Add 'next_run_at'
);
-- Seed Pipelines data
INSERT INTO pipelines
VALUES(
        'pipeline1',
        '*/1 * * * *'
    ),
    (
        'pipeline2',
        '*/2 * * * *'
    );

--------------------------------------
-- Create Tasks table and seed data --
--------------------------------------
CREATE TABLE IF NOT EXISTS tasks (
    id TEXT NOT NULL PRIMARY KEY,
    pipeline_id TEXT NOT NULL,
    command TEXT NOT NULL
    -- TODO: Add created_at
    -- TODO: Add modified_at
);
-- Seed Tasks data
INSERT INTO tasks
VALUES(
        'task1',
        'pipeline1',
        'sleep 6 && echo some logs'
    ),
    (
        'task2',
        'pipeline1',
        'sleep 11'
    ),
    (
        'task3',
        'pipeline2',
        'sleep 8 && fail'
    ),
    (
        'task4',
        'pipeline2',
        'sleep 7'
    );


-----------------------------------------------
-- Create the PipelineRuns table and seed data --
-----------------------------------------------
CREATE TABLE IF NOT EXISTS task_instances (
    id TEXT NOT NULL PRIMARY KEY
    , task_id TEXT NOT NULL
    , pipeline_id TEXT NOT NULL
    , execution_time TEXT NOT NULL
    , status TEXT NOT NULL
    , logs TEXT NOT NULL
    , created_at TEXT NOT NULL
);
