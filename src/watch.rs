use crate::machine_status::save_status;
use dynomite::dynamodb::DynamoDbClient;
use job_scheduler::{Job, JobScheduler};
use std::error::Error;
use std::time::Duration;

pub fn start(client: &DynamoDbClient, table_name: &String) -> Result<(), Box<Error>> {
    let mut sched = JobScheduler::new();
    let foo = "5 * * * * *".parse()?;

    sched.add(Job::new(foo, || match save_status(&client, &table_name) {
        Ok(output) => println!("Status saved {:?}", output),
        Err(error) => println!("Put item error: {:?}", error),
    }));

    // Save status immediately (once)
    save_status(&client, &table_name).unwrap();

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(10000));
    }
}
