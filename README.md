# kv

A key-value store.

## Examples

### Run the server

```
cargo run --bin server
```

### Run the CLI client

Set a key-value pair:

```
cargo run --bin cli put foo bar
```

Get a value by key:

```
cargo run --bin cli get foo
```