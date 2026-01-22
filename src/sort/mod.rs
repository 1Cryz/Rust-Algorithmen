/// Bubble Sort: wiederholt benachbarte Elemente tauschen,
/// bis kein Tausch mehr nötig ist.
pub fn bubble_sort(values: &mut [i32]) {
    let mut swapped = true;
    let mut n = values.len();

    while swapped {
        swapped = false;
        // Jede Runde schiebt das grösste Element ans Ende.
        for i in 1..n {
            if values[i - 1] > values[i] {
                values.swap(i - 1, i);
                swapped = true;
            }
        }
        // Nach jeder Runde ist das grösste Element hinten korrekt.
        n = n.saturating_sub(1);
    }
}

/// Quick Sort: Pivot wählen, partitionieren und rekursiv sortieren.
pub fn quick_sort(values: &mut [i32]) {
    if values.len() <= 1 {
        return;
    }

    // Partitionieren: alle Werte < Pivot nach links, > Pivot nach rechts.
    let pivot_index = partition(values);
    let (left, right) = values.split_at_mut(pivot_index);
    quick_sort(left);
    // Das Pivot-Element liegt an Position 0 des rechten Teilbereichs.
    quick_sort(&mut right[1..]);
}

fn partition(values: &mut [i32]) -> usize {
    let len = values.len();
    let pivot = values[len - 1];
    let mut store_index = 0;

    for i in 0..len - 1 {
        if values[i] < pivot {
            // Kleine Elemente vor den Pivot schieben.
            values.swap(i, store_index);
            store_index += 1;
        }
    }

    // Pivot an die endgültige Position bringen.
    values.swap(store_index, len - 1);
    store_index
}

/// Merge Sort: Liste teilen, rekursiv sortieren und zusammenführen.
pub fn merge_sort(values: &mut [i32]) {
    let len = values.len();
    if len <= 1 {
        return;
    }

    // Liste halbieren, rekursiv sortieren.
    let mid = len / 2;
    merge_sort(&mut values[..mid]);
    merge_sort(&mut values[mid..]);

    // Zusammenführen der sortierten Hälften.
    let mut merged = Vec::with_capacity(len);
    let (left, right) = values.split_at(mid);
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    // Restliche Elemente anhängen.
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);

    // Ergebnis zurück in das Original-Array kopieren.
    values.copy_from_slice(&merged);
}
