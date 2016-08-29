use std::collections::BTreeSet;

fn main() {
    let mut cache = BTreeSet::new();
    cache.insert(1);

    converges(80, &mut cache);
    converges(7, &mut cache);
}

fn collatz(number: u64) -> u64 {
    match number % 2 == 0 {
        false => 3 * number + 1,
        true => number / 2,
    }
}

fn converges(number: u64, cache: &mut BTreeSet<u64>) -> bool {
    let mut seen = BTreeSet::new();
    let mut converged = false;
    let mut current = number;

    print!("{0}: {0}", number);

    while !cache.contains(&current) {
        seen.insert(current);
        current = collatz(current);
        print!(" -> {}", current);

        if seen.contains(&current) {
            break;
        }

        if current == 1 {
            converged = true;
        }
    }

    if converged {
        print!(" (converged)");
        cache.append(&mut seen);
    } else {
        print!(" (diverged)");
    }

    println!("");

    converged
}

#[cfg(test)]
mod test {
    #[test]
    fn collatz_step() {
        assert_eq!(collatz(7), 22);
        assert_eq!(collatz(22), 11);
        assert_eq!(collatz(11), 34);
        assert_eq!(collatz(34), 17);
        assert_eq!(collatz(17), 52);
        assert_eq!(collatz(52), 26);
        assert_eq!(collatz(26), 13);
        assert_eq!(collatz(13), 40);
        assert_eq!(collatz(40), 20);
        assert_eq!(collatz(20), 10);
        assert_eq!(collatz(10), 5);
        assert_eq!(collatz(5), 16);
        assert_eq!(collatz(16), 8);
        assert_eq!(collatz(8), 4);
        assert_eq!(collatz(4), 2);
        assert_eq!(collatz(2), 1);
    }
}
