# Rust bindings for runh CLI

A crate for consuming the runh binary in your Rust applications.

## Usage
Both sync/async version is available.
You can build runh client with `RunhConfig` in method chaining style.
Call `build()` or `build_async()` to get client.
Note that async client depends on [tokio](https://github.com/tokio-rs/tokio), then please use it on tokio runtime.

```rust,ignore
#[tokio::main]
async fn main() {
    let config = runh::GlobalOpts::new()
        .root("./new_root")
        .debug(false)
        .log("/path/to/logfile.json")
        .log_format(runh::LogFormat::Json)
        .rootless(true);

    let client = config.build_async().unwrap();

    let opts = runh::options::CreateOpts::new()
        .pid_file("/path/to/pid/file")
        .no_pivot(true);

    client.create("container-id", "path/to/bundle", Some(&opts)).unwrap();
}
```

## Limitations
- Supported commands are only:
    - create
    - start
    - state
    - kill
    - delete
- Exec is **not** available in `RunhAsyncClient` now.
- Console utilites are **not** available
