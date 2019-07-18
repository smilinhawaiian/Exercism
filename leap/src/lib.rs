pub fn is_leap_year(year: u64) -> bool {
    let mut is_leap = false;
    if year % 4 == 0 {
        // year is leap year if evenly divisible by 4
        if year % 100 == 0 && year % 400 != 0 {
            // except each year evenly divisible by 100
            is_leap = false;
        } else {
            // unless evenly divisible by 400
            is_leap = true;
        }
    }
    is_leap
}
