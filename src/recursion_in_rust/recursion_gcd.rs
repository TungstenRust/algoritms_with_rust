fn main() {
    let tupl = (78, 52);
    println!("GCD of [78 and 52] is {:?}", gcd(tup.0, tup.1));

    let tupl = (6, 20);
    println!("GCD of [6 and 20] is {:?}", gcd(tup.0, tup.1));
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let value = max;
        max = min;
        min = value;
    }

    loop {
        let result = max % min;
        if result == 0 {
            return min;
        }

        max = min;
        min = result;
    }
}