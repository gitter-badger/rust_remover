# rust_remover
This is an Discord bot written in Rust.
It is mainly for personal usage.

If you want to use it for some reason go see the installation instructions.

## Installation

### Dependencies
- Rust nightly v1.20 (may work with stable. not tested)  
- All rust build dependencys  
- At least OpenSSL 1.1.0f  

### Linux
```bash
$ git clone https://github.com/HeapUnderfl0w/rust_remover.git
$ cd rust_remover
$ cargo build
```

### Windows

```batch
> git clone https://github.com/HeapUnderfl0w/rust_remover.git
> cd rust_remover
> cargo build --no-default-features
```
 The `--no-default-features` flag is used to disable the `psutil` crate which failes to build under Windows.
 
 ## Usage
 Enviroment variables
 
 | Name                | Description                                                      | Required |  
 | ------------------- | ---------------------------------------------------------------- | :------: |  
 | DISCORD_TOKEN       | Your Discord app token                                           | YES      |  
 | CLEVERBOT_TOKEN     | Your cleverbot API token                                         | YES¹     |  
 | RUST_REMOVER_LOG4RS | Path to the Logging configuration file (default: log4rs.yaml)².  | NO       | 
 
 ¹) This will be optional when the Cleverbot feature is fully implemented as an feature.  
 ²) On the yaml format & usage see the [log4rs documentation](https://docs.rs/log4rs/0.7.0/log4rs/#examples)
