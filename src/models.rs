use dynomite::Item;
use std::cmp::Ordering;

#[derive(Item, Debug, Clone, Eq)]
pub struct MachineStatus {
    #[hash]
    pub status_id: String,
    pub machine_id: String,
    pub time_to_exist: i64,
    pub mac_address: String,
    pub timestamp: String,
    pub status_meta: String,
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
