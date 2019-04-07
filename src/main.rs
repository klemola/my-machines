mod machine_status;

fn main() {
    use machine_status::{get_client, list_status, save_status};

    let table_name = String::from("machine-status");
    let client = get_client();

    match list_status(&client, &table_name) {
        Ok(status_list) => {
            println!("Status list {:?}", status_list);
        }
        Err(error) => {
            println!("Could not list status: {:?}", error);
        }
    }

    match save_status(&client, &table_name) {
        Ok(output) => {
            println!("Status saved {:?}", output);
        }
        Err(error) => {
            println!("Put item error: {:?}", error);
        }
    }
}
