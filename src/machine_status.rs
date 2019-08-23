use crate::models::MachineStatus;

use crate::system::{hostname, mac_address, system_summary};
use chrono::{DateTime, Duration, Utc};
use dynomite::{
    dynamodb::{DynamoDb, DynamoDbClient, PutItemError, PutItemInput, PutItemOutput, ScanInput},
    FromAttributes,
};
use itertools::Itertools;
use prettytable::{Cell, Row, Table};
use rusoto_core::{Region, RusotoError};
use std::error::Error;
use uuid::Uuid;

pub fn get_client() -> DynamoDbClient {
    DynamoDbClient::new(Region::EuNorth1)
}

pub fn list(
    client: &DynamoDbClient,
    table_name: &str,
) -> Result<Vec<MachineStatus>, Box<dyn Error>> {
    let scan_input = ScanInput {
        table_name: String::from(table_name),
        select: Some(String::from("ALL_ATTRIBUTES")),
        ..Default::default()
    };

    let output = client.scan(scan_input).sync()?;
    let items = output.items.unwrap_or_default();
    let status_vec = items
        .into_iter()
        .map(|result| MachineStatus::from_attrs(result))
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

pub fn save(
    client: &DynamoDbClient,
    table_name: &str,
) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    let utc: DateTime<Utc> = Utc::now();
    let two_days_from_now = utc.checked_add_signed(Duration::days(2)).unwrap_or(utc);

    let machine_status = MachineStatus {
        status_id: Uuid::new_v4().to_string(),
        machine_id: hostname(),
        mac_address: mac_address(),
        timestamp: utc.to_rfc3339(),
        status_meta: system_summary(),
        time_to_exist: two_days_from_now.timestamp(),
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
