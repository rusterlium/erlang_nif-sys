# erlang_nif-sys
[![](http://meritbadge.herokuapp.com/erlang_nif-sys)](https://crates.io/crates/erlang_nif-sys)
[![Docs](https://docs.rs/erlang_nif-sys/badge.svg)](https://docs.rs/erlang_nif-sys)
[![Build Status](https://travis-ci.org/goertzenator/erlang_nif-sys.svg?branch=master)](https://travis-ci.org/goertzenator/erlang_nif-sys)
[![Build status](https://ci.appveyor.com/api/projects/status/rssa03e29mxou4hv/branch/master?svg=true)](https://ci.appveyor.com/project/goertzenator/erlang-nif-sys/branch/master)

# **This repo is no longer in use. This code is now maintained as part of the main [rustler](github.com/rusterlium/rustler) repo.**

A crate for creating [Erlang NIF modules](http://www.erlang.org/doc/man/erl_nif.html) in Rust.  This crate exposes the raw C NIF API which can be used directly or as a foundation for higher layer interface crates.  Supported under Unix and Windows.

See the [crate documention](https://docs.rs/erlang_nif-sys).

See examples of use:
 - [rust.mk](https://github.com/goertzenator/rust.mk) for a sample Rust NIF module.
 - [rebar3_rust](https://github.com/sdwolf/rebar3_rust) a rebar3 plugin inspired by `rust.mk` that helps integrate Rust code inside Erlang projects.
 - [Rustler](https://github.com/hansihe/Rustler)
 - [rustfromerl](https://github.com/sdwolf/rustfromerl) a demo project showing performance differences between Erlang code and a simmilar Rust NIF implementation.

Thanks go to Radosław Szymczyszyn for bootstrapping me on this Rust FFI adventure and providing the original [automatic bindings](https://github.com/lavrin/erlang-rust-nif/blob/master/rust_src/src/c.rs).
