# Rust-Algorithmen

Dieses Projekt implementiert grundlegende Algorithmen in Rust und stellt sie
über eine einfache CLI bereit. Alle Algorithmen sind kommentiert, sodass die
Funktionsweise direkt im Code nachvollzogen werden kann.

## Anforderungen (Checkliste)

- **Alle Algorithmen implementiert und beschrieben:**
  - Sieb des Eratosthenes (single-threaded, multithreaded, async/await)
  - Bubble Sort
  - Quick Sort
  - Merge Sort
  - Binary Search
- **Ausführbares Rust-Projekt:** `cargo run` erzeugt ein Binary.
- **Separate Module:** `src/sieve/mod.rs`, `src/sort/mod.rs`, `src/search/mod.rs`.
- **CLI-Steuerung:** Jeder Algorithmus kann über ein Subcommand gestartet werden.

## Algorithmen im Überblick

### Sieb des Eratosthenes
Markiert alle Zahlen bis zu einem Limit als "mögliche Primzahlen" und entfernt
systematisch die Vielfachen jeder gefundenen Primzahl. Das Ergebnis ist die
Liste aller Primzahlen bis zum angegebenen Grenzwert.

- **`sieve-single`**: Single-Threaded Variante.
- **`sieve-thread`**: Bereichsweise Parallelisierung über `thread::spawn`.
- **`sieve-async`**: Ausführung in einem `async`-Block (async/await).

### Bubble Sort
Beim Bubble Sort werden immer zwei benachbarte Elemente miteinander verglichen.
Wenn sie in der falschen Reihenfolge sind, werden sie vertauscht.
Dieser Vorgang wird mehrmals wiederholt, bis alles richtig sortiert ist.
Dabei wandern die grössten Elemente Schritt für Schritt nach hinten.

### Quick Sort
Beim Quick Sort wird zuerst ein Element ausgewählt, mit dem die anderen verglichen werden.
Alle kleineren Elemente kommen auf die linke Seite und alle grösseren auf die rechte Seite.
Danach werden die beiden Seiten auf die gleiche Weise weiter sortiert.
So wird die Liste nach und nach geordnet.

### Merge Sort
Beim Merge Sort wird die Liste immer wieder in zwei Hälften geteilt.
Das passiert so lange, bis nur noch einzelne Elemente übrig sind.
Anschliessend werden diese Elemente wieder zusammengeführt.
Dabei werden sie direkt in der richtigen Reihenfolge zusammengesetzt.

### Binary Search
Bei der Binary Search wird in einer bereits sortierten Liste gesucht.
Zuerst wird das mittlere Element angeschaut.
Je nachdem, ob das gesuchte Element grösser oder kleiner ist, wird nur noch in einer Hälfte weitergesucht.
Dieser Vorgang wiederholt sich, bis das gesuchte Element gefunden wurde oder nicht existiert.

## CLI Usage

```bash
cargo run -- <command> [args]
```

### Beispiele mit Output

#### Sieb des Eratosthenes (single-threaded)
```bash
cargo run -- sieve-single 30
```
Output:
```
Primzahlen bis 30: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
```

#### Sieb des Eratosthenes (threaded)
```bash
cargo run -- sieve-thread 30 4
```
Output:
```
Primzahlen bis 30 (4 Threads): [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
```

#### Sieb des Eratosthenes (async/await)
```bash
cargo run -- sieve-async 30
```
Output:
```
Primzahlen bis 30 (async): [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
```

#### Bubble Sort
```bash
cargo run -- bubble 9 1 5 3
```
Output:
```
Bubble Sort Ergebnis: [1, 3, 5, 9]
```

#### Quick Sort
```bash
cargo run -- quick 9 1 5 3
```
Output:
```
Quick Sort Ergebnis: [1, 3, 5, 9]
```

#### Merge Sort
```bash
cargo run -- merge 9 1 5 3
```
Output:
```
Merge Sort Ergebnis: [1, 3, 5, 9]
```

#### Binary Search
```bash
cargo run -- binary 1 3 5 7 9 7
```
Output:
```
Binary Search in [1, 3, 5, 7, 9] nach 7: Some(3)
```