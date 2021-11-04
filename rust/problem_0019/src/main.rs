fn problem_19() -> u32 {
    let mut total_sundays = 0;
    let mut weekday = 1;
    // There are 4 months which are 30 days long:
    // September, April, June and November.
    // The rest of the 7 months are 31 days long,
    // except February which has 28 days except on leap years.
    let mut month_lengths = vec![
        31, // January
        0,  // February
        31, // March
        30, // April
        31, // May
        30, // June
        31, // July
        31, // August
        30, // September
        31, // October
        30, // November
        31 // December
    ];

    for year in 1901..=2000 {
        // Since 2000 is divisible by 400, it's therefor also divisible
        // by 4, so there's no need to apply that rule here.
        month_lengths[1] = if year % 4 == 0 {
            29
        } else {
            28
        };

        for month in 0..=11 {
            if weekday % 7 == 6 {
                total_sundays += 1
            }
            weekday += month_lengths[month];
        }
    }

    total_sundays
}

#[test]
fn test_sundays() {
    assert_eq!(problem_19(), 171);
}
