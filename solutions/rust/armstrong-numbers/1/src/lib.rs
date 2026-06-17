pub fn is_armstrong_number(num: u32) -> bool {
   
    if num == 0 {
        return true;
    }

    let power = num.ilog10() + 1;
    let num_str = num.to_string();
    let mut total_sum = 0;

    for d in num_str.chars() {
        // Convert the character digit (e.g., '4') into a u32 number (4)
        if let Some(digit) = d.to_digit(10) {
            // Raise the digit to the power of the total number of digits
            total_sum += digit.pow(power);
        }
    }

    // It's an Armstrong number if the accumulated sum equals the original number
    total_sum == num
}