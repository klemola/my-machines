use dynomite::{
    dynamodb::{DynamoDb, DynamoDbClient, PutItemError, PutItemInput, PutItemOutput, ScanInput},
    FromAttributes, Item,
};
use hostname::get_hostname;
use itertools::Itertools;
use rusoto_core::Region;
use std::error::Error;
use time;
use uuid::Uuid;

#[derive(Item, Debug, Clone)]
pub struct MachineStatus {
    #[hash]
    status_id: String,
    machine_id: String,
    timestamp: String,
    status_meta: String,
}

pub fn get_client() -> DynamoDbClient {
    return DynamoDbClient::new(Region::EuNorth1);
}

pub fn list_status(
    client: &DynamoDbClient,
    table_name: &String,
) -> Result<Vec<MachineStatus>, Box<Error>> {
    let scan_input = ScanInput {
        table_name: table_name.clone(),
        select: Some(String::from("ALL_ATTRIBUTES")),
        ..Default::default()
    };

    let output = client.scan(scan_input).sync()?;
    let items = output.items.unwrap_or(Vec::new());
    let status_vec = items
        .iter()
        .map(|result| MachineStatus::from_attrs(result.clone()))
        .filter_map(|mapped| mapped.ok())
        .unique_by(|status| status.machine_id.clone())
        .collect::<Vec<MachineStatus>>();

    return Ok(status_vec);
}

pub fn save_status(
    client: &DynamoDbClient,
    table_name: &String,
) -> Result<PutItemOutput, PutItemError> {
    let hostname = get_hostname().unwrap_or_default();

    let machine_status = MachineStatus {
        status_id: Uuid::new_v4().to_string(),
        machine_id: hostname.into(),
        timestamp: time::now_utc().rfc3339().to_string(),
        status_meta: String::from("Something"),
    };

    let put_item_input = PutItemInput {
        item: machine_status.clone().into(),
        table_name: table_name.clone(),
        ..Default::default()
    };

    return client.put_item(put_item_input).sync();
}
