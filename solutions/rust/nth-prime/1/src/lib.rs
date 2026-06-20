pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut candidate = 2;

    loop {
        // Check if 'candidate' is prime
        let mut is_prime = true;
        let mut i = 2;
        
        // Arithmetic check up to the square root of candidate
        while i * i <= candidate {
            if candidate % i == 0 {
                is_prime = false;
                break;
            }
            i += 1;
        }

        // If it's prime, check if it's the one we want
        if is_prime {
            if count == n {
                return candidate;
            }
            count += 1;
        }

        candidate += 1;
    }
}
