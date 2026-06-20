pub fn factors(n: u64) -> Vec<u64> {
    let mut divs = Vec::new();
    let mut divisor = 2;
    let mut rem = n;

    while rem > 1 {
        if rem % divisor == 0 {
            divs.push(divisor);
            rem /= divisor;
        }else {
            divisor += 1;
        }
    }
    divs
}
