Fixerio
=========================
[![Build Status](https://img.shields.io/travis/sb89/fixerio/master.svg)](https://travis-ci.org/sb89/fixerio)
[![License](https://img.shields.io/github/license/sb89/fixerio.svg)]()
[![Crates.io](https://img.shields.io/crates/v/fixerio.svg)](https://crates.io/crates/fixerio)
[![Docs.rs](https://docs.rs/fixerio/badge.svg)](https://docs.rs/fixerio)

A Rust wrapper for the Fixerio API.

## Installation
Add the following to `Cargo.toml`:

```toml
[dependencies]
fixerio = "0.1.0"
```

## Usage
Synchronous example:

```rust,no_run
extern crate fixerio;
 
use fixerio::{Config, Currency, SyncApi};
use std::time::Duration;

fn main() {
  let api = SyncApi::new().expect("Error creating API");

  let mut config = Config::new(Currency::USD);
  let rates = api.get(&config).expect("Error retrieving rates");

  println!("{:?}", rates);
}
```

Asynchronous example:

```rust,no_run

extern crate fixerio;
extern crate tokio_core;

use fixerio::{Config, Currency, AsyncApi};
use tokio_core::reactor::Core;
use std::time::Duration;

fn main() {
  let mut core = Core::new().expect("Error creating core");
  let handle = core.handle();

  let api = AsyncApi::new(&handle);

  let config = Config::new(Currency::USD);
  let work = api.get(&config);

  println!("{:?}", core.run(work).expect("Error retrieving rates"));
}
```
