extern crate hostname;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use hostname::get_hostname;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput};
use std::collections::HashMap;

struct MachineStatus {
    machine_id: String,
    status_id: u8,
}

fn main() {
    let client = DynamoDbClient::new(Region::EuNorth1);

    let hostname = get_hostname().unwrap_or_default();

    let machine_status = MachineStatus {
        machine_id: hostname,
        status_id: 0,
    };

    let mut item_input: HashMap<String, AttributeValue> = HashMap::new();

    item_input.insert(
        String::from("MachineID"),
        AttributeValue {
            s: Some(machine_status.machine_id),
            ..Default::default()
        },
    );

    item_input.insert(
        String::from("StatusID"),
        AttributeValue {
            n: Some(machine_status.status_id.to_string()),
            ..Default::default()
        },
    );

    let put_item_input = PutItemInput {
        item: item_input,
        table_name: String::from("machine-status"),
        ..Default::default()
    };

    match client.put_item(put_item_input).sync() {
        Ok(output) => {
            println!("Status saved {:?}", output);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
