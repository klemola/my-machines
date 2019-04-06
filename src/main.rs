extern crate dynomite;
extern crate hostname;
extern crate rusoto_core;

use dynomite::{
    dynamodb::{DynamoDb, DynamoDbClient, PutItemInput},
    retry::Policy,
    Item, Retries,
};
use hostname::get_hostname;
use rusoto_core::Region;

#[derive(Item, Debug, Clone)]
struct MachineStatus {
    #[hash]
    MachineID: String,
    StatusID: u16,
    StatusMeta: String,
}

fn main() {
    let client = DynamoDbClient::new(Region::EuNorth1).with_retries(Policy::default());;
    let hostname = get_hostname().unwrap_or_default();
    let table_name = String::from("machine-status");

    let machine_status = MachineStatus {
        MachineID: hostname.into(),
        StatusID: 0,
        StatusMeta: String::from("Something"),
    };

    let put_item_input = PutItemInput {
        item: machine_status.clone().into(),
        table_name: table_name.clone(),
        ..Default::default()
    };

    match client.put_item(put_item_input).sync() {
        Ok(output) => {
            println!("Status saved {:?}", output);
        }
        Err(error) => {
            println!("Put item error: {:?}", error);
        }
    }
}
