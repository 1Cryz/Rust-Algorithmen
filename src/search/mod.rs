/// Binary Search: funktioniert nur auf sortierten Arrays.
/// Prüft jeweils das mittlere Element und halbiert den Suchbereich.
pub fn binary_search(values: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = values.len();

    while low < high {
        // Mitte berechnen und Vergleich durchführen.
        let mid = low + (high - low) / 2;
        match values[mid].cmp(&target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    // Nicht gefunden.
    None
}
