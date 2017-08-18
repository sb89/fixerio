//! # Fixerio API Wrapper
//!
//! http://fixer.io
//!
//! ## Usage
//! Add the following to `Cargo.toml`:
//!
//! ```rust,ignore
//! [dependencies]
//! fixerio = "0.1.0"
//! ```
//!
//! Synchronous example:
//!
//! ```rust,no_run
//!
//! extern crate fixerio;
//!
//! use fixerio::{Config, Currency, SyncApi};
//!
//! fn main() {
//!     let api = SyncApi::new().expect("Error creating API");
//!
//!     let config = Config::new(Currency::USD);
//!     let rates = api.get(&config).expect("Error retrieving rates");
//!
//!     println!("{:?}", rates);
//! }
//! ```
//!
//! Asynchronous example:
//!
//! ```rust,no_run
//!
//! extern crate fixerio;
//! extern crate tokio_core;
//!
//! use fixerio::{Config, Currency, AsyncApi};
//! use tokio_core::reactor::Core;
//!
//! fn main() {
//!     let mut core = Core::new().expect("Error creating core");
//!     let handle = core.handle();
//!
//!     let api = AsyncApi::new(&handle);
//!
//!     let config = Config::new(Currency::USD);
//!     let work = api.get(&config);
//!
//!     println!("{:?}", core.run(work).expect("Error retrieving rates"));
//! }
//! ```

#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate tokio_core;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate error_chain;

mod client;
mod exchange;
mod future;
mod errors;

pub use self::client::{AsyncApi, SyncApi, Config};
pub use self::exchange::{Currency, Exchange, Rates};
pub use self::future::FixerioFuture;