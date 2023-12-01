// TODO
// This should be a single process that is responsible for the following:
// 1. Seeing that something is ready to be run in the database
// 2. Spawning a thread to handle execution
// 3. Checking the status?

pub fn run() {
    // Load Pipelines into memory

    // Loop through them to figure out when the next runtime is

    // If it is ready to run, pass it to an execution thread

    // check the results of the execution thread, store the logs/status in the database
    todo!()
}
