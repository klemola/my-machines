use crate::models::MachineStatus;
use dynomite::{
    dynamodb::{DynamoDb, DynamoDbClient, PutItemError, PutItemInput, PutItemOutput, ScanInput},
    FromAttributes,
};
use hostname::get_hostname;
use itertools::Itertools;
use mac_address::get_mac_address;
use prettytable::{Cell, Row, Table};
use rusoto_core::Region;
use std::error::Error;
use time;
use uuid::Uuid;

pub fn get_client() -> DynamoDbClient {
    DynamoDbClient::new(Region::EuNorth1)
}

pub fn list(client: &DynamoDbClient, table_name: &str) -> Result<Vec<MachineStatus>, Box<Error>> {
    let scan_input = ScanInput {
        table_name: table_name.to_string(),
        select: Some(String::from("ALL_ATTRIBUTES")),
        ..Default::default()
    };

    let output = client.scan(scan_input).sync()?;
    let items = output.items.unwrap_or_default();
    let status_vec = items
        .into_iter()
        .map(|result| MachineStatus::from_attrs(result.clone()))
        .filter_map(std::result::Result::ok)
        .sorted_by(|s1, s2| Ord::cmp(&s2, &s1))
        .unique_by(|status| status.mac_address.clone())
        .collect::<Vec<MachineStatus>>();

    Ok(status_vec)
}

pub fn list_and_handle_result(client: &DynamoDbClient, table_name: &str) {
    let result = list(&client, &table_name);

    match result {
        Ok(status_list) => {
            let mut table = Table::new();
            let mapped_status = status_list
                .into_iter()
                .map(|ms| {
                    vec![
                        Cell::new(&ms.machine_id),
                        Cell::new(&ms.mac_address),
                        Cell::new(&ms.timestamp),
                        Cell::new(&ms.status_meta),
                    ]
                })
                .collect::<Vec<Vec<Cell>>>();

            table.add_row(row!["Machine ID", "Mac address", "Timestamp", "Meta",]);

            for ms in mapped_status {
                table.add_row(Row::new(ms));
            }

            table.printstd();
        }
        Err(error) => println!("Could not list status: {:?}", error),
    }
}

pub fn save(client: &DynamoDbClient, table_name: &str) -> Result<PutItemOutput, PutItemError> {
    let hostname = get_hostname().unwrap_or_default();
    let m_address = match get_mac_address() {
        Ok(Some(address)) => address.to_string(),
        _ => "".to_string(),
    };
    let now = time::now_utc();

    let machine_status = MachineStatus {
        status_id: Uuid::new_v4().to_string(),
        machine_id: hostname,
        mac_address: m_address,
        timestamp: now.rfc3339().to_string(),
        status_meta: String::from("Something"),
    };

    let put_item_input = PutItemInput {
        item: machine_status.clone().into(),
        table_name: table_name.to_string(),
        ..Default::default()
    };

    client.put_item(put_item_input).sync()
}

pub fn save_and_handle_result(client: &DynamoDbClient, table_name: &str) {
    let result = save(&client, &table_name);

    match result {
        Ok(_) => println!("Status saved"),
        Err(error) => println!("Put item error: {:?}", error),
    }
}
