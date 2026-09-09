# tinydb

Storage engine viết từ đầu bằng Rust, **không dùng thư viện ngoài nào** — mục `[dependencies]`
trong `Cargo.toml` để trống. Mục tiêu là hiểu database hoạt động ra sao ở tầng dưới cùng: trang dữ
liệu nằm trên đĩa thế nào, B+Tree tìm khoá ra sao, WAL đảm bảo không mất dữ liệu bằng cách nào.

Kiến trúc bám theo **CMU 15-445 / BusTub**, viết lại theo cách idiomatic của Rust.

```
$ cargo run
tinydb> SQL REPL — type 'exit' to quit
Commands: INSERT INTO t (id, name) VALUES (1, "alice")
          SELECT * FROM t WHERE id = 1
```

---

## Kiến trúc

```
┌─────────────────────────────────────────────────┐
│                main.rs (REPL)                   │
│              lex → parse → execute              │
└─────────────────────┬───────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────┐
│                table.rs (Table)                 │
│   ┌─────────────┐        ┌─────────────────┐    │
│   │   heap.rs   │        │    btree.rs     │    │
│   │   (Heap)    │        │  (BPlusTree)    │    │
│   │  Pages +    │        │  Arena-based    │    │
│   │  File I/O   │        │  Non-clustered  │    │
│   └─────────────┘        └─────────────────┘    │
└─────────────────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────┐
│                 page.rs (Page)                  │
│           Slotted page: 8192 bytes              │
│       Item pointers ← | → Tuple data            │
└─────────────────────────────────────────────────┘
```

| Module | Nội dung |
|---|---|
| `page.rs` | Slotted page 8192 byte — item pointer mọc từ đầu, dữ liệu tuple mọc từ cuối |
| `heap.rs` | Heap nhiều trang, quản lý free space, đọc ghi file |
| `btree.rs` | B+Tree **arena-based** (`Vec<NodeId>` thay cho con trỏ), lá nối nhau để quét dải |
| `row.rs` | Mã hoá và giải mã bản ghi |
| `table.rs` | Table = Heap + B+Tree, index không phân cụm |
| `query.rs` | Lexer và parser SQL tự viết |
| `executor.rs` | Thực thi AST xuống các thao tác của Table |
| `wal.rs` | Write-Ahead Log và Transaction |
| `error.rs` | `DbError` |

---

## Đã có

- **Slotted page 8 KB** — chèn, đọc, quản lý free space; xoá bằng tombstone (offset = 0)
- **Heap nhiều trang** — ghi thẳng ra file
- **B+Tree** kiểu arena, tách node theo tham số `order`, lá liên kết qua `next: Option<NodeId>`
- **Durability** — `Table::open()` dựng lại index bằng REINDEX khi mở
- **SQL** — lexer và parser tự viết cho `INSERT` và `SELECT ... WHERE`
- **Executor + REPL**
- **WAL và Transaction** — commit, rollback
- **38 test** — `cargo test` xanh toàn bộ

```
$ cargo test
test result: ok. 38 passed; 0 failed; 0 ignored
```

## Chưa có

Đây là dự án học, không phải database dùng cho việc thật. Những phần còn thiếu so với BusTub:

- **Buffer Pool Manager** — hiện đọc ghi thẳng file, chưa có page guard hay chính sách thay trang
- **Concurrency control** — chưa có latch, chưa có MVCC, chạy một luồng
- **Query optimizer** — executor chạy thẳng AST, không có kế hoạch thực thi hay ước lượng chi phí
- **`UPDATE` và `DELETE`** ở tầng SQL — heap có `delete`, nhưng parser chưa nhận hai lệnh này
- **Crash recovery đầy đủ** — có WAL nhưng chưa có ARIES redo/undo sau sự cố

---

## Đối chiếu với CMU 15-445 / BusTub

| Thành phần | BusTub (C++) | tinydb (Rust) |
|---|---|---|
| Storage | Buffer Pool Manager + Page Guards | Đọc ghi file trực tiếp + Heap page |
| B+Tree | Header page + leaf/internal page | Arena-based (`Vec<NodeId>`) |
| Tách node | `leaf_max_size`, `internal_max_size` | tham số `order` |
| Tombstone | Leaf tombstone buffer | Page offset = 0 |
| Iterator | Index iterator | lá nối nhau qua `next: Option<NodeId>` |
| SQL Parser | Lexer + Parser | `query.rs` |
| Executor | AST → thao tác Table | `executor.rs` |
| WAL | Write-Ahead Logging | `wal.rs` + Transaction |
| Durability | Crash recovery | `Table::open()` REINDEX |

Chi tiết từng checkpoint và ghi chú thiết kế nằm trong [`SPEC.md`](SPEC.md).

---

## Chạy thử

```bash
cargo test          # 38 test
cargo run           # REPL
```

```sql
INSERT INTO t (id, name) VALUES (1, "alice")
SELECT * FROM t WHERE id = 1
```

---

Ngô Tấn Đạt — [github.com/tandat8896](https://github.com/tandat8896)
