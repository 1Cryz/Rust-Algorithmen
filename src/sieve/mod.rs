use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::thread;

/// Berechnet Primzahlen bis zu einer Grenze mit dem klassischen Sieb.
/// Idee: Alle Zahlen als "möglich prim" markieren und Vielfache entfernen.
pub fn sieve_single(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }

    // is_prime[i] == true bedeutet: i gilt aktuell als prim.
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2;
    while p * p <= limit {
        if is_prime[p] {
            // Alle Vielfachen von p sind nicht prim.
            // Wir starten bei p*p, da kleinere Vielfache bereits markiert wurden.
            for multiple in (p * p..=limit).step_by(p) {
                is_prime[multiple] = false;
            }
        }
        p += 1;
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(n, &prime)| if prime { Some(n) } else { None })
        .collect()
}

/// Multithreaded-Variante mit `thread::spawn`.
/// Vorgehen: Basis-Primzahlen bis sqrt(limit) bestimmen und anschließend
/// den Wertebereich in Blöcke aufteilen. Jeder Thread markiert sein Segment.
pub fn sieve_threaded(limit: usize, threads: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }

    // Mindestens ein Thread, damit die Logik funktioniert.
    let threads = threads.max(1);
    // Basis-Primzahlen werden nur bis sqrt(limit) benötigt.
    let base_limit = (limit as f64).sqrt() as usize;
    let base_primes = sieve_single(base_limit);

    let shared = Arc::new(Mutex::new(vec![true; limit + 1]));
    {
        let mut guard = shared.lock().expect("mutex lock");
        guard[0] = false;
        guard[1] = false;
    }

    let chunk_size = (limit + 1 + threads - 1) / threads;
    let mut handles = Vec::with_capacity(threads);

    for chunk_index in 0..threads {
        // Jedes Segment ist ein zusammenhängender Bereich [start, end).
        let start = chunk_index * chunk_size;
        let end = min(start + chunk_size, limit + 1);
        let base_primes = base_primes.clone();
        let shared = Arc::clone(&shared);

        let handle = thread::spawn(move || {
            if start >= end {
                return;
            }

            // Lokales Segment, um Lock-Contention zu vermeiden.
            let mut local = vec![true; end - start];

            for &p in &base_primes {
                // Startmultiplikator im Segment finden.
                // Wir springen auf das erste Vielfache im Segment.
                let mut multiple = (start + p - 1) / p * p;
                if multiple < p * p {
                    multiple = p * p;
                }

                for m in (multiple..end).step_by(p) {
                    local[m - start] = false;
                }
            }

            let mut guard = shared.lock().expect("mutex lock");
            for (offset, &prime) in local.iter().enumerate() {
                let value = start + offset;
                if value < 2 {
                    // 0 und 1 sind nicht prim.
                    guard[value] = false;
                } else if !prime {
                    guard[value] = false;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("thread join");
    }

    let guard = shared.lock().expect("mutex lock");
    guard
        .iter()
        .enumerate()
        .filter_map(|(n, &prime)| if prime { Some(n) } else { None })
        .collect()
}

/// Async/await Variante: Berechnung wird in einem async Block ausgeführt.
pub async fn sieve_async(limit: usize) -> Vec<usize> {
    let primes = async { sieve_single(limit) }.await;
    primes
}
