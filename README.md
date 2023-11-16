# Synthesizer

Synthesizer is a lightweight cron-based task scheduler. It is designed to be completely decoupled from the code it is executing.

It aims to have as many of the (useful) features of other task orchestration frameworks (Airflow, DAGster, Prefect) without tying you to a specific language (Python) and encouraging more decoupled execution styles.

## Proposed Design

As the development of the MVP is still in-progress, the following is a "final vision" of what it is expected to look like. This is a pseudo design document guiding the development of the project

For better separation of concerns, the `server` and `cli` are developed as discrete crates, but with intertwined dependencies.

### Server

Syntherizer has a lightweight server with three distinct logical components:

1. A REST API - This is used for registering new pipelines, listing pipelines, etc.
2. A Scheduler - This component will continually check the current time against the scheduled next time of each pipeline. When a pipeline is "ready" for execution, it is marked as `ready` in the database.
3. An Executor - This is what actually allows the server to execute the scheduled tasks. This would be a pool of worker threads (or a single async thread polling remote execution?). It polls the database for tasks that are `ready` and then executes them accordingly.

### CLI

To facilitate a better user-experience, a lightweight CLI wrapper around the API server will be created. It is expected that the CLI will be talking to a remote instance and therefore is configurable.

### UI

There will be a relatively simple HTMX front-end to make it easier to interact with the server. However, this does not mean that the CLI is a second-class citizen and should also have a great user experience for interacting with the server.

### Workflow

1. A user would define a `pipeline` in YAML format, providing the required fields.
2. They would then use the CLI to `register` the pipeline(s) with the server. Something like `syn register`. It is considered best practice to keep _all_ pipelines in a file in version control to make it easier to keep track of what pipelines are registered.
3. The server does an `upsert` with the registered pipelines
4. The scheduler does its thing, eventually marking the pipelines as `ready`
5. The executor picks up the new pipeline.
6. The pipeline's tasks are then run either via local worker threads or remotely. Ideally, the remote execution mode would be "infinitely scalable" by leveraging async/await to poll for remote work completion. This would remove the need to have a thread-per-task execution model.
