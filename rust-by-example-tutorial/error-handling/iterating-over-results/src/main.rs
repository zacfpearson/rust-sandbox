fn main() {
    let strings = vec!["tofu", "93", "18"];

    //ignore failed items
    let numbers_ignore: Vec<_> = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    println!("Results: {:?}", numbers_ignore);

    //fail on any error
    let numbers_fail: Result<Vec<_>, _> = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers_fail);

    //collect all valid values and failures with partition
    let (numbers_partition, errors_partition): (Vec<_>, Vec<_>) = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers_partition);
    println!("Errors: {:?}", errors_partition);

    let (numbers_partition_clean, errors_partition_clean): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers_partition_clean: Vec<_> = numbers_partition_clean.into_iter().map(Result::unwrap).collect();
    let errors_partition_clean: Vec<_> = errors_partition_clean.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers_partition_clean);
    println!("Errors: {:?}", errors_partition_clean);
}
