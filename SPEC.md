# OpenSpec: tinydb Database Engine

Aligned with CMU 15-445 BusTub architecture. Build as you learn, profile with Flamelens.

## Current Status

```
✅ Checkpoint 1: error.rs — DbError enum
✅ Checkpoint 2: row.rs — Row encode/decode
✅ Checkpoint 3: page.rs — Slotted page 8192B
✅ Checkpoint 4: heap.rs — Multi-page heap
✅ Checkpoint 5: btree.rs — Arena-based B+Tree
✅ Checkpoint 6: table.rs — Table = Heap + BPlusTree
✅ Checkpoint 7: durability — Table::open() REINDEX
✅ Checkpoint 8: query.rs — SQL lexer/parser
✅ Checkpoint 9: executor.rs + REPL
✅ Checkpoint 10: wal.rs — WAL + Transaction
✅ 38 tests pass
```

## CMU 15-445 Alignment

| Component | BusTub (C++) | tinydb (Rust) | Status |
|-----------|--------------|---------------|--------|
| **Storage** | Buffer Pool Manager + Page Guards | Direct file I/O + Heap pages | ✅ Simplified |
| **B+Tree** | Header page + leaf/internal pages | Arena-based (Vec<NodeId>) | ✅ Idiomatic Rust |
| **Split** | `leaf_max_size`, `internal_max_size` | `order` parameter | ✅ Same concept |
| **Tombstone** | Leaf tombstone buffer | Page offset=0 | ✅ Simple |
| **Iterator** | Index iterator | `next: Option<NodeId>` linked leaves | ✅ |
| **SQL Parser** | Lexer + Parser | query.rs | ✅ |
| **Executor** | AST → Table ops | executor.rs | ✅ |
| **WAL** | Write-Ahead Logging | wal.rs + Transaction | ✅ |
| **Durability** | Crash recovery | Table::open() REINDEX | ✅ |

## Architecture

```
┌─────────────────────────────────────────────────┐
│                   main.rs (REPL)                │
│         lex → parse → execute                   │
└─────────────────────┬───────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────┐
│              table.rs (Table)                    │
│   ┌─────────────┐     ┌─────────────────┐      │
│   │   heap.rs   │     │    btree.rs     │      │
│   │   (Heap)    │     │  (BPlusTree)    │      │
│   │  Pages +    │     │  Arena-based    │      │
│   │  File I/O   │     │  Non-clustered  │      │
│   └─────────────┘     └─────────────────┘      │
└─────────────────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────┐
│              page.rs (Page)                      │
│   Slotted page: 8192 bytes                      │
│   Item pointers ← | → Tuple data                │
└─────────────────────────────────────────────────┘
```

## Flamelens Profiling

```bash
# Build with debug symbols
cargo build --release

# Generate test input
for i in $(seq 1 500); do echo "INSERT INTO users (id, name) VALUES ($i, \"user_$i\")"; done > /tmp/commands.txt
echo "exit" >> /tmp/commands.txt

# Record perf (pin P-core 0-7)
taskset -c 0-7 perf record --call-graph dwarf -F 9999 -o /tmp/perf.data -- ./target/release/tinydb < /tmp/commands.txt

# Convert + view
perf script -i /tmp/perf.data | inferno-collapse-perf > /tmp/tinydb.folded
flamelens /tmp/tinydb.folded
```

## Profiling Results (500 inserts)

| Function | Samples | % | Notes |
|----------|---------|---|-------|
| `btree::insert_data` | 408K | 26% | B+Tree split operations |
| `query::parse::to_uppercase` | 409K | 26% | String allocation |
| Memory alloc (`RawVec::grow`) | 321K | 21% | Vec growth |
| `query::lex` | 246K | 16% | String parsing |
| `heap::insert` | 78K | 5% | File I/O |

## Optimization Opportunities

1. **`parse()` — `to_uppercase()`** → Use `eq_ignore_ascii_case()` instead
2. **`lex()` — Pre-allocate** → `Vec::with_capacity(16)`
3. **B+Tree — Vec capacity** → `Vec::with_capacity(order)` when creating nodes
4. **Memory — Reduce clones** → Use `&str` references where possible

---

## Checkpoints (Detailed)

### Checkpoint 1: error.rs

```rust
#[derive(Debug)]
pub enum DbError {
    Io(std::io::Error),
    Parse(String),
    NotFound(String),
    InvalidQuery(String),
}
impl From<std::io::Error> for DbError { ... }
```

### Checkpoint 2: row.rs

```rust
pub struct Row {
    pub id: i64,
    pub fields: Vec<String>,
}
impl Row {
    pub fn encode(&self) -> Vec<u8> { ... }
    pub fn decode(bytes: &[u8]) -> Result<Self, DbError> { ... }
}
```

### Checkpoint 3: page.rs

```rust
pub const PAGE_SIZE: usize = 8192;
pub struct Page {
    pub data: [u8; PAGE_SIZE],
    num_slots: u16,
    free_start: u16,
    free_end: u16,
}
```

### Checkpoint 4: heap.rs

```rust
pub type Tid = (u32, u16);
pub struct Heap {
    file: std::fs::File,
    pages: Vec<Page>,
}
```

### Checkpoint 5: btree.rs

```rust
type NodeId = usize;
enum Node<K, V> {
    Internal { keys: Vec<K>, children: Vec<NodeId> },
    Leaf { keys: Vec<K>, values: Vec<V>, next: Option<NodeId> },
}
pub struct BPlusTree<K: Ord + Clone, V> {
    arena: Vec<Node<K, V>>,
    root: NodeId,
    order: usize,
}
```

### Checkpoint 6: table.rs

```rust
pub struct Table {
    heap: Heap,
    index: BPlusTree<i64, Tid>,
}
```

### Checkpoint 7: durability

```rust
impl Table {
    pub fn open(path: &str) -> Result<Self, DbError> {
        // REINDEX: scan heap, rebuild B+Tree
    }
}
```

### Checkpoint 8: query.rs

```rust
pub enum Stmt {
    Insert { table: String, id: i64, fields: Vec<String> },
    Select { table: String, where_id: Option<i64> },
}
pub fn lex(input: &str) -> Vec<String> { ... }
pub fn parse(tokens: &[String]) -> Result<Stmt, DbError> { ... }
```

### Checkpoint 9: executor.rs

```rust
pub fn execute(stmt: Stmt, table: &mut Table) -> Result<String, DbError> { ... }
```

### Checkpoint 10: wal.rs

```rust
pub struct Wal { file: BufWriter<File> }
pub struct Transaction<'a> { table, wal, pending: Vec<Row> }
```

---

## Future Extensions (Checkpoint 11)

1. **B+Tree delete + merge** — Rebalance when delete
2. **Concurrency** — RwLock for multi-client
3. **VACUUM** — Free space reclaim
4. **MVCC** — Tuple header for snapshot isolation
