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
To use Cleverbot append the flag `--features "cleverbot"` to the `cargo build` command.  
If cleverbot is enabled, the `CLEVERBOT_TOKEN` enviroment variable is needed.  

### Windows

```batch
> git clone https://github.com/HeapUnderfl0w/rust_remover.git
> cd rust_remover
> cargo build --no-default-features
```
 The `--no-default-features` flag is used to disable the `psutil` crate which failes to build under Windows.
 
 ## Usage
 Enviroment variables
 
 | Name                  | Description                             | Required                     | Default        |
 | --------------------- | --------------------------------------- | :--------------------------: | -------------- |
 | `DISCORD_TOKEN`       | Your Discord app token                  | YES                          |                |
 | `CLEVERBOT_TOKEN`     | Your cleverbot API token                | YES with `cleverbot` feature |                |
 | `RUST_REMOVER_LOG4RS` | Path to the Logging configuration file. | NO                           | `log4rs.yaml`¹ |
 
 ¹) On the yaml format & usage see the [log4rs documentation](https://docs.rs/log4rs/0.7.0/log4rs/#examples)
 
 ## Note
 I work on this project for fun (aka shits & giggles).  
 #### What you can expect:
 - Updates
 - I (try) to answer as many questions as i can
 - Solved issues
 
 #### What you can't expect:
- Great Proficiency with either Git or Rust
- Good "commits" (no multifile commits, etc.)

If you have questions or want to contribute to the bot feel free to ask !
