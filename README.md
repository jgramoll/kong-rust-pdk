# Kong Rust PDK

## example

Start plugin server
```
cargo run --bin helloworld
```

Send command from kong to run plugin
```
cargo run -p kong-mock
```


## Local dev

debug macro
```
cargo expand --bin helloworld
```
 