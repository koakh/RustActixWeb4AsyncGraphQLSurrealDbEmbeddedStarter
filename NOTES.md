# NOTES

## TLDR

```shell
$ screen
$ cd ~/.tiup/
# tikv
$ ./start.sh
# outcome
PD client endpoints: [127.0.0.1:2379 127.0.0.1:2382 127.0.0.1:2384]
To view the Prometheus: http://127.0.0.1:41871
To view the Grafana: http://127.0.0.1:3000
# or 
$ bin/tiup playground --tag surrealdb-kv --mode tikv-slim --pd 3 --kv 3

# new window
$ cd ~/.surrealdb/
$ ./sdbstart.sh 
# outcome
[2022-10-29 22:14:33] INFO  surrealdb::iam Root authentication is enabled
[2022-10-29 22:14:33] INFO  surrealdb::iam Root username is 'root'
[2022-10-29 22:14:33] INFO  surrealdb::dbs Database strict mode is disabled
[2022-10-29 22:14:33] INFO  surrealdb::kvs Connecting to kvs store at tikv://127.0.0.1:2379
[2022-10-29 22:14:33] INFO  surrealdb::kvs Connected to kvs store at tikv://127.0.0.1:2379
[2022-10-29 22:14:33] INFO  surrealdb::net Starting web server on 0.0.0.0:8000
[2022-10-29 22:14:33] INFO  surrealdb::net Started web server on 0.0.0.0:8000
# or
# ./surreal start --log debug --user root --pass root tikv://127.0.0.1:2379

# project
$ cd ~/Development/examples4_async_graphql/
```

## Links

### ActixWeb

- project started from actixweb [examples4_async_graphql](https://github.com/async-graphql/examples/tree/master/actix-web/starwars)

### SurrealDB

- [surrealdb](https://docs.rs/surrealdb/1.0.0-beta.8/surrealdb/)

### Rust

- [Rust from_into](https://doc.rust-lang.org/rust-by-example/conversion/from_into.html)

## Install SurrealDB Crate

```shell
$ cargo add surrealdb  --features kv-tikv
    Updating crates.io index
      Adding surrealdb v1.0.0-beta.8 to dependencies.
             Features:
             + http
             + kv-mem
             + kv-rocksdb
             + kv-tikv
             + parallel
             + scripting
             - foundationdb
             - kv-fdb
             - kv-fdb-5_1
             - kv-fdb-5_2
             - kv-fdb-6_0
             - kv-fdb-6_1
             - kv-fdb-6_2
             - kv-fdb-6_3
             - kv-fdb-7_0
             - kv-fdb-7_1
             - kv-indxdb
```

## KoakhLaptop

```shell
$ cargo build
error: failed to run custom build command for `grpcio-sys v0.8.1`
```

- <https://discord.com/channels/902568124350599239/970338835990974484/1011994237602111609>

- **Tobie at SurrealDB — 24/08/2022**

Ok @chrisb / @koakh I've done a few things:

1. Updated the https://github.com/surrealdb/surrealdb/blob/main/doc/BUILDING.md doc
2. Created a new tikv branch which incorporates a minor change of code, and uses the master branch of the tikv-client dependency (which uses the latest release version of grpcio-sys).
3. Asked the maintainers if they plan on releasing a new version which incorporates many of their bug fixes and updates: https://github.com/tikv/client-rust/issues/365

- [Building SurrealDB](https://github.com/surrealdb/surrealdb/blob/main/doc/BUILDING.md)

```shell
# Compile for x86_64-unknown-linux-gnu
$ cargo build --release --locked --target x86_64-unknown-linux-gnu
```

now gives

```
  CMake Warning at cmake/protobuf.cmake:51 (message):
    gRPC_PROTOBUF_PROVIDER is "module" but PROTOBUF_ROOT_DIR is wrong
```

- https://github.com/datenlord/datenlord/issues/221

- https://github.com/tikv/tikv/issues/3748#issuecomment-436524368

I find the fault, cmake version must be >3.8 now. Thank you very very much,bro!

Prerequisites

CMake >= 3.8.0
Rust >= 1.19.0

By default, the secure feature is enabled, therefore Go (>=1.7) is required.

```shell
❯ cmake --version
cmake version 3.24.2
```

## In KoakhLaptop

```shell
    Building [=======================> ] 586/587: async-graphql-demo(bin)                                                                                                                                      
    Finished dev [unoptimized + debuginfo] target(s) in 7m 13s
```

## In KoakhServer

```shell
$ sudo apt-get -y install \
	curl \
	llvm \
	cmake \
	binutils \
	clang-11 \
	qemu-user \
	musl-tools \
	libssl-dev \
	pkg-config \
	build-essential
```
