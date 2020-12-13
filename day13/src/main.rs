fn main() {
    let target_timestamp = 1001171;
    let bus_ids = [17,41,37,367,19,23,29,613,13];

    //let target_timestamp = 939;
    //let bus_ids = [7, 13, 59, 31, 19];

    let mut minutes_waiting: Vec<(i32, i32)> = bus_ids
        .iter()
        .map(
            |&bus_id| (bus_id, ((target_timestamp / bus_id) + 1) * bus_id)
        )
        .collect();

    minutes_waiting.sort_by(|sched, other| sched.1.partial_cmp(&other.1).unwrap());

    println!("Minutes waiting: {:?}. Multiplied: {}", minutes_waiting[0], minutes_waiting[0].0 * (minutes_waiting[0].1 - target_timestamp));
}
