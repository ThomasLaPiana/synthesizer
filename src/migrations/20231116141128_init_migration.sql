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
        'echo Beginning task1 for pipeline1! && sleep 6 && echo Ending task1 for pipeline1!'
    ),
    (
        'task2',
        'pipeline1',
        'echo Beginning task2 for pipeline1! && sleep 7 && echo Ending task2 for pipeline1!'
    ),
    (
        'task3',
        'pipeline2',
        'echo Beginning task1 for pipeline2! && sleep 8 && echo Ending task1 for pipeline2!'
    ),
    (
        'task4',
        'pipeline2',
        'echo Beginning task2 for pipeline2! && sleep 9 && echo Ending task2 for pipeline2!'
    );

-------------------------------------
-- Create the Pipeline Queue table --
-------------------------------------
CREATE TABLE IF NOT EXISTS pipeline_queue (
    id TEXT NOT NULL PRIMARY KEY
    , pipeline_id TEXT NOT NULL
    , schedule
    , command TEXT NOT NULL
    , created_at TEXT NOT NULL
    , modified_at TEXT NOT NULL
    , picked_up BOOLEAN NOT NULL
);

-----------------------------------------------
-- Create the PipelineRuns table and seed data --
-----------------------------------------------
CREATE TABLE IF NOT EXISTS pipeline_runs (
    id TEXT NOT NULL PRIMARY KEY,
    task_id TEXT NOT NULL,
    pipeline_id TEXT NOT NULL,
    status TEXT
    -- TODO: Add created_at
    -- TODO: Add modified_at
);
