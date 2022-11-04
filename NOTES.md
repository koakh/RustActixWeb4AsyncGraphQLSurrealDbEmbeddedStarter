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
# surrealdb
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

# new window, THIS will connect to same TiKV instance as GraphQL server, this way we can use shell
$ cd ~/.surrealdb/
$ ./sdbsql.sh
# or
./surreal sql --conn http://localhost:8000 --user root --pass root --ns test --db test --pretty

# add some seed data

CREATE person:tobie CONTENT { name: 'Tobie', meta_data: { field: 'some joe tobie' } };
CREATE person:jamie CONTENT { name: 'Jamie', meta_data: { field: 'some joe jamie' } };
CREATE person:koakh CONTENT { name: 'Koakh', meta_data: { field: 'some joe koakh' } };
CREATE person:pelu CONTENT { name: 'Pelo', meta_data: { field: 'some joe pelu' } };
CREATE person:jomo CONTENT { name: 'Jomo', meta_data: { field: 'some joe jomo' } };
CREATE person:funi CONTENT { name: 'Funi', meta_data: { field: 'some joe funi' } };
CREATE person:joni CONTENT { name: 'Joni', meta_data: { field: 'some joe joni' } };
CREATE person:devy CONTENT { name: 'Devy', meta_data: { field: 'some joe devy' } };
CREATE person:peti CONTENT { name: 'Peti', meta_data: { field: 'some joe peti' } };
CREATE person:andy CONTENT { name: 'Andy', meta_data: { field: 'some joe andy' } };
CREATE person:hulk CONTENT { name: 'Hulk', meta_data: { field: 'some joe hulk' } };

# project
$ cd ~/Development/examples4_async_graphql/
$ make run
# outcome
[2022-10-29T21:15:39Z TRACE actix_server::worker] Service "actix-web-service-0.0.0.0:8282" is available
[2022-10-29T21:15:39Z TRACE actix_server::signals] setting up OS signal listener
```

now open <http://localhost:8282/playground>

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

## Development Links

## BTreeMap

- https://doc.rust-lang.org/nightly/alloc/collections/btree_map/struct.BTreeMap.html







TODO: use parameters and Thing
https://discordapp.com/channels/902568124350599239/1014970959461105664/1036062437222404167


TODO: 
empty record set
https://discordapp.com/channels/902568124350599239/1014970959461105664/1036066184518447264

The Response Value is in every case an Value::Array. If you want to select an single record by its id using select $ID or whatever you would get an array which contains just the single record. If the record does not exist the array would be empty. For handling this just do value.into_iter().next()



TODO: 
sugestion of code improvement from BLucky
https://discordapp.com/channels/902568124350599239/1036366380968194128/1036545636172054608

```rust
pub async fn create_user(
    username: String,
    password: String,
    ds: Datastore,
    sess: &Session,
) -> Result<String> {
    let sql = "CREATE $id set password = crypto::argon2::generate($password)";
    let vars = Vars::from([
        ("id".into(), thing(&format!("user:{username}"))?.into()),
        ("password".into(), password.into()),
    ]);
    println!("{vars:#?}");
    let ress = ds.execute(sql, sess, Some(vars), false).await?;
    println!("{ress:#?}");
    Ok("no_id".into())
}
```

TODO: 
sugestion of code improvement from BLucky
https://discordapp.com/channels/902568124350599239/1014970959461105664/1036552022394142771

```rust
let mut ast = "SELECT * FROM person".to_string();
// init parameters btree
let mut vars = BTreeMap::new();
if let Some(f) = filter {
    let mut filter_fields: Vec<&str> = Vec::new();
    if let Some(v) = &f.id {
        filter_fields.push("id = $id");
        vars.insert(
            "id".to_string(),
            Value::Thing(Thing {
                tb: "person".to_string(),
                id: { Id::String(v.to_string()) },
            }),
        );
    }
    if let Some(v) = &f.name {
        filter_fields.push("name = $name");
        vars.insert("name".to_string(), Value::Strand(Strand(v.to_string())));
    }
    if let Some(v) = &f.age {
        filter_fields.push("age = $age");
        vars.insert("age".to_string(), Value::Number(Number::Int(*v as i64)));
    }
    for (i, el) in filter_fields.iter().enumerate() {
        if i == 0 {
            ast.push_str(" WHERE ");
        }
        if i > 0 {
            ast.push_str(" AND ");
        }
        ast.push_str(el);
    }
}
```

by the way you can use `surrealdb::sql::thing("table:id")` instead of manually constructing `Value::Thing`
(example: `vars.insert("id".into(), thing(&format!("person:"{v}"))?.into()` to create a `String` of `person:` and the contents of v, then get a reference to it to implicitly cast the String into &str, and then pass it to `thing()`, handle the `Result` and then cast the value with `.into())`

## Relay Pagination 

- https://github.com/async-graphql/async-graphql/issues/974#issuecomment-1192284485
- https://github.com/azzamsa/tin
