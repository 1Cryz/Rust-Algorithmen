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
Vergleicht benachbarte Elemente und vertauscht sie bei Bedarf. Mehrere Durchläufe
schieben schrittweise die grössten Elemente nach hinten.

### Quick Sort
Wählt ein Pivot-Element, partitioniert die Liste (kleiner links, grösser rechts)
und sortiert die Teilbereiche rekursiv.

### Merge Sort
Teilt die Liste rekursiv in Hälften, sortiert jede Hälfte und führt sie danach
wieder zusammen (Merge-Schritt).

### Binary Search
Sucht in einer **sortierten** Liste durch wiederholtes Halbieren des Suchbereichs.

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