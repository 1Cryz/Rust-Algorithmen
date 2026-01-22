mod search;
mod sieve;
mod sort;

use futures::executor::block_on;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    // Der erste Parameter bestimmt, welcher Algorithmus ausgeführt wird.
    // Dadurch kann jede Aufgabe separat über die CLI gestartet werden.
    match args[1].as_str() {
        "sieve-single" => {
            let limit = parse_usize(args.get(2)).unwrap_or(100);
            let primes = sieve::sieve_single(limit);
            println!("Primzahlen bis {}: {:?}", limit, primes);
        }
        "sieve-thread" => {
            let limit = parse_usize(args.get(2)).unwrap_or(100);
            let threads = parse_usize(args.get(3))
                .or_else(|| std::thread::available_parallelism().ok().map(|n| n.get()))
                .unwrap_or(4);
            let primes = sieve::sieve_threaded(limit, threads);
            println!(
                "Primzahlen bis {} ({} Threads): {:?}",
                limit, threads, primes
            );
        }
        "sieve-async" => {
            let limit = parse_usize(args.get(2)).unwrap_or(100);
            let primes = block_on(sieve::sieve_async(limit));
            println!("Primzahlen bis {} (async): {:?}", limit, primes);
        }
        "bubble" => {
            let mut values = parse_numbers(&args[2..]);
            if values.is_empty() {
                // Fallback-Daten, falls keine Zahlen übergeben werden.
                values = vec![5, 3, 8, 4, 2];
            }
            sort::bubble_sort(&mut values);
            println!("Bubble Sort Ergebnis: {:?}", values);
        }
        "quick" => {
            let mut values = parse_numbers(&args[2..]);
            if values.is_empty() {
                // Fallback-Daten, falls keine Zahlen übergeben werden.
                values = vec![5, 3, 8, 4, 2];
            }
            sort::quick_sort(&mut values);
            println!("Quick Sort Ergebnis: {:?}", values);
        }
        "merge" => {
            let mut values = parse_numbers(&args[2..]);
            if values.is_empty() {
                // Fallback-Daten, falls keine Zahlen übergeben werden.
                values = vec![5, 3, 8, 4, 2];
            }
            sort::merge_sort(&mut values);
            println!("Merge Sort Ergebnis: {:?}", values);
        }
        "binary" => {
            let mut values = parse_numbers(&args[2..]);
            let target = if values.len() >= 2 {
                // Letzter Wert wird als Suchziel interpretiert.
                values.pop().expect("target vorhanden")
            } else {
                // Fallback, wenn keine Werte übergeben wurden.
                values = vec![1, 3, 5, 7, 9, 11, 13];
                7
            };
            // Binary Search benötigt eine sortierte Liste.
            values.sort();
            let result = search::binary_search(&values, target);
            println!(
                "Binary Search in {:?} nach {}: {:?}",
                values, target, result
            );
        }
        _ => print_usage(),
    }
}

fn parse_usize(value: Option<&String>) -> Option<usize> {
    value.and_then(|text| text.parse::<usize>().ok())
}

fn parse_numbers(args: &[String]) -> Vec<i32> {
    args.iter()
        .filter_map(|value| value.parse::<i32>().ok())
        .collect()
}

fn print_usage() {
    println!(
        "Usage: cargo run -- <command> [args]\n\n\
Commands (Beispiele):\n\
  sieve-single <limit>              Sieb des Eratosthenes (ein Thread)\n\
                                    cargo run -- sieve-single 100\n\
  sieve-thread <limit> <threads>    Sieb des Eratosthenes (thread::spawn)\n\
                                    cargo run -- sieve-thread 100 4\n\
  sieve-async <limit>               Sieb des Eratosthenes (async/await)\n\
                                    cargo run -- sieve-async 100\n\
  bubble <numbers...>               Bubble Sort\n\
                                    cargo run -- bubble 9 1 5 3\n\
  quick <numbers...>                Quick Sort\n\
                                    cargo run -- quick 9 1 5 3\n\
  merge <numbers...>                Merge Sort\n\
                                    cargo run -- merge 9 1 5 3\n\
  binary <numbers...> <target>      Binary Search (letztes Argument = Ziel)\n\
                                    cargo run -- binary 1 3 5 7 9 7\n"
    );
}
