use crate::machine_status::save_and_handle_result;
use dynomite::dynamodb::DynamoDbClient;
use job_scheduler::{Job, JobScheduler};
use std::error::Error;
use std::time::Duration;

pub fn start(client: &DynamoDbClient, table_name: &str) -> Result<(), Box<Error>> {
    let mut sched = JobScheduler::new();
    let sched_interval = "5 * * * * *".parse()?;

    sched.add(Job::new(sched_interval, || {
        save_and_handle_result(&client, &table_name)
    }));

    // Save status immediately (once)
    save_and_handle_result(&client, &table_name);

    // ... and then at intervals
    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(10000));
    }
}
