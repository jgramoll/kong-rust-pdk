# Kong Rust PDK

## example

Start plugin server
```
cargo run --bin helloworld
```

Send command from kong to run plugin
```
cargo run --bin kong_mock
```


## Local dev

debug macro
```
cargo expand --bin helloworld
```
 