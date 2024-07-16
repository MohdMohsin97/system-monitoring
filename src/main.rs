

mod data_collection; 

fn main() {
    loop {
        // First snapshot
        data_collection::cpu_usage_percentage();
        data_collection::mem_usage_percentage();
    }
}