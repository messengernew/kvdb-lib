# Key: Value database library

A simple thread-safe key-value storage library using DashMap.

## Usage

Add `kvdb-lib` to your `Cargo.toml`:

```toml
[dependencies]
kvdb-lib = "0.1.6"
```

Add the following to your main file:

```rust
use kvdb_lib::Storage;

fn main() {
    let storage = Storage::new();
    storage.set(1, "value1");
    println!("{:?}", storage.get(&1));
}
```

## Examples

### Creating a Storage

Create a new `Storage` instance:

```rust
use kvdb_lib::Storage;

let storage: Storage<i32, &str> = Storage::new();
```

### Setting a Key-Value Pair

Insert a key-value pair:

```rust
use kvdb_lib::Storage;

let storage = Storage::new();
storage.set(1, "value1");
```

### Getting a Value

Retrieve a value by its key:

```rust
use kvdb_lib::Storage;

let storage = Storage::new();
storage.set(1, "value1");

assert_eq!(storage.get(&1), Some("value1"));
```

### Removing a Key-Value Pair

Remove a key-value pair:

```rust
use kvdb_lib::Storage;

let storage = Storage::new();
storage.set(1, "value1");
storage.remove(1);

assert_eq!(storage.get(&1), None);
```

### Getting All Key-Value Pairs

Retrieve all key-value pairs:

```rust
use kvdb_lib::Storage;

let storage = Storage::new();

storage.set(1, "value1");
storage.set(2, "value2");
let all = storage.get_all();

assert!(all.contains(&(1, "value1")));
assert!(all.contains(&(2, "value2")));
```

## License

This project is licensed under the MIT License.