extern crate fern;
#[macro_use(error, info, log)]
extern crate log;

use std::collections::BTreeSet;

fn main() {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            format!("[{:>5}] {}", level, msg)
        }),
        output: vec![fern::OutputConfig::stdout()],
        level: log::LogLevelFilter::Info,
    };
    let log = fern::init_global_logger(logger_config, log::LogLevelFilter::Info);
    if let Err(e) = log {
        panic!("error initializing log: {}", e);
    }

    let mut cache = BTreeSet::new();
    cache.insert(1);

    for i in 1..100 {
        if !converges(i, &mut cache) {
            error!("found a divergent sequence for {}", i);
            break;
        }
    }
}

fn collatz(number: u64) -> u64 {
    match number % 2 == 0 {
        false => 3 * number + 1,
        true => number / 2,
    }
}

fn converges(number: u64, cache: &mut BTreeSet<u64>) -> bool {
    let mut seen = Vec::new();
    let mut current = number;
    loop {
        if cache.contains(&current) {
            seen.push(current);
            info!("  {}: {:?} (converged)", number, seen);
            cache.extend(seen);
            return true;
        }

        if seen.contains(&current) {
            seen.push(current);
            error!("! {}: {:?} (diverged)", number, seen);
            return false;
        }

        seen.push(current);
        current = collatz(current);
    }
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
