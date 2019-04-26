use dynomite::{
    dynamodb::{DynamoDb, DynamoDbClient, PutItemError, PutItemInput, PutItemOutput, ScanInput},
    FromAttributes, Item,
};
use hostname::get_hostname;
use itertools::Itertools;
use mac_address::get_mac_address;
use rusoto_core::Region;
use std::cmp::Ordering;
use std::error::Error;
use time;
use uuid::Uuid;

#[derive(Item, Debug, Clone, Eq)]
pub struct MachineStatus {
    #[hash]
    status_id: String,
    machine_id: String,
    mac_address: String,
    timestamp: String,
    status_meta: String,
}

impl Ord for MachineStatus {
    fn cmp(&self, other: &MachineStatus) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for MachineStatus {
    fn partial_cmp(&self, other: &MachineStatus) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MachineStatus {
    fn eq(&self, other: &MachineStatus) -> bool {
        self.machine_id == other.machine_id
    }
}

pub fn get_client() -> DynamoDbClient {
    DynamoDbClient::new(Region::EuNorth1)
}

pub fn list_status(
    client: &DynamoDbClient,
    table_name: &str,
) -> Result<Vec<MachineStatus>, Box<Error>> {
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

pub fn save_status(
    client: &DynamoDbClient,
    table_name: &str,
) -> Result<PutItemOutput, PutItemError> {
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
