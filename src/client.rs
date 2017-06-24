use hyper::client::{Client, HttpConnector};
use hyper;
use tokio_core::reactor::{Core, Handle, Timeout};
use futures::future::{Future, result};
use futures::Stream;
use serde_json;

use std::cell::RefCell;
use std::str;
use std::time::Duration;

use exchange::{Currency, Exchange};
use future::FixerioFuture;
use errors::Error;

/// Configuration for requests.
pub struct Config {
    base: Currency,
    symbols: Option<Vec<Currency>>,
    date: Option<String>,
}

impl Config {
    /// Create a new Config struct.
    pub fn new(curr: Currency) -> Self {
        Self {
            base: curr,
            symbols: None,
            date: None,
        }
    }

    /// Set the symbols (List of specific exchange rates for the base currency, will retrieve all if none as specified).
    pub fn set_symbols(&mut self, symbols: Vec<Currency>) {
        self.symbols = Some(symbols);
    }

    /// Set the date (The date for which to retrieve exchange rates from, will retrieve latest if a date is not specified).
    pub fn set_date<T: Into<String>>(&mut self, date: T) {
        self.date = Some(date.into());
    }
}

fn get_url(conf: &Config) -> hyper::Uri {
    let mut url = String::from("http://api.fixer.io/");

    match conf.date {
        Some(ref date) => url.push_str(&date),
        None => url.push_str("latest"),
    };

    url.push_str("?base=");
    url.push_str(conf.base.string());

    if let Some(ref currencies) = conf.symbols {
        let string_vec: Vec<&str> = currencies.iter().map(|x| x.string()).collect();
        let sep_string = string_vec.join(",");

        url.push_str("&symbols=");
        url.push_str(&sep_string);
    }

    url.parse::<hyper::Uri>().unwrap()
}

/// Asynchronous API for sending requests
pub struct AsyncApi {
    client: Client<HttpConnector>,
    handle: Handle,
}

impl AsyncApi {
    /// Create a new instance.
    pub fn new(handle: &Handle) -> Self {
        Self {
            client: Client::new(&handle),
            handle: handle.clone(),
        }
    }

    /// Perform request with timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    ///
    /// # extern crate fixerio;
    /// # extern crate tokio_core;
    /// use fixerio::{Config, Currency, AsyncApi};
    /// use tokio_core::reactor::Core;
    /// use std::time::Duration;
    ///
    /// fn main() {
    ///     let mut core = Core::new().expect("Error creating core");
    ///     let handle = core.handle();
    ///
    ///     let api = AsyncApi::new(&handle);
    ///
    ///     let config = Config::new(Currency::USD);
    ///
    ///     let work = api.get_timeout(&config, Duration::from_secs(5));
    ///
    ///     let rates = core.run(work).expect("Error retrieving rates");
    ///
    ///     match rates {
    ///         Some(x) => println!("{:?}", x),
    ///         _ => println!("Request timed out")
    ///     };
    /// }
    /// ```
    pub fn get_timeout(&self,
                       conf: &Config,
                       duration: Duration)
                       -> FixerioFuture<Option<Exchange>> {
        let timeout_future = result(Timeout::new(duration, &self.handle))
            .flatten()
            .map_err(From::from)
            .map(|_| None);

        let get = self.get(conf).map(|x| Some(x));

        let future = timeout_future
            .select(get)
            .map(|(item, _next)| item)
            .map_err(|(item, _next)| item);

        FixerioFuture::new(Box::new(future))
    }

    /// Perform request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    ///
    /// # extern crate fixerio;
    /// # extern crate tokio_core;
    /// use fixerio::{Config, Currency, AsyncApi};
    /// use tokio_core::reactor::Core;
    /// use std::time::Duration;
    ///
    /// fn main() {
    ///     let mut core = Core::new().expect("Error creating core");
    ///     let handle = core.handle();
    ///
    ///     // Get the latest exchange rates for USD
    ///     let api = AsyncApi::new(&handle);
    ///
    ///     let config = Config::new(Currency::USD);
    ///     let work = api.get(&config);
    ///
    ///     println!("{:?}", core.run(work).expect("Error retrieving rates"));
    /// }
    /// ```
    pub fn get(&self, conf: &Config) -> FixerioFuture<Exchange> {
        let url = get_url(conf);

        let future =
            self.client
                .get(url)
                .and_then(|res| res.body().concat2())
                .map_err(From::from)
                .and_then(|bytes| result(serde_json::from_slice(&bytes).map_err(From::from)));

        FixerioFuture::new(Box::new(future))
    }
}

/// Synchronous API for sending requests.
pub struct SyncApi {
    core: RefCell<Core>,
    api: AsyncApi,
}

impl SyncApi {
    /// Create a new instance.
    pub fn new() -> Result<Self, Error> {
        let core = Core::new()?;
        let handle = core.handle();

        Ok(Self {
               core: RefCell::new(core),
               api: AsyncApi::new(&handle),
           })
    }

    /// Perform request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    ///
    /// # extern crate fixerio;
    /// use fixerio::{Config, Currency, SyncApi};
    /// use std::time::Duration;
    ///
    /// fn main() {
    ///     let api = SyncApi::new().expect("Error creating API");
    ///
    ///     let config = Config::new(Currency::USD);
    ///
    ///     let rates = api.get(&config).expect("Error retrieving rates");
    ///
    ///     println!("{:?}", rates);
    /// }
    /// ```
    pub fn get(&self, conf: &Config) -> Result<Exchange, Error> {
        let future = self.api.get(conf);

        self.core.borrow_mut().run(future)
    }

    /// Perform request with timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    ///
    /// # extern crate fixerio;
    /// use fixerio::{Config, Currency, SyncApi};
    /// use std::time::Duration;
    ///
    /// fn main() {
    ///     let api = SyncApi::new().expect("Error creating API");
    ///
    ///     let config = Config::new(Currency::USD);
    ///
    ///     let rates = api.get_timeout(&config, Duration::from_secs(5)).expect("Error retrieving rates");
    ///
    ///     match rates {
    ///         Some(x) => println!("{:?}", x),
    ///         _ => println!("Request timed out")
    ///     };
    /// }
    /// ```
    pub fn get_timeout(&self,
                       conf: &Config,
                       duration: Duration)
                       -> Result<Option<Exchange>, Error> {
        let future = self.api.get_timeout(conf, duration);

        self.core.borrow_mut().run(future)
    }
}

#[cfg(test)]
mod tests {
    use client::{Config, get_url};
    use exchange::Currency;

    #[test]
    fn get_url_test() {
        let tests = vec![("http://api.fixer.io/latest?base=AUD",
                          Config {
                              base: Currency::AUD,
                              symbols: None,
                              date: None,
                          }),
                         ("http://api.fixer.io/latest?base=EUR",
                          Config {
                              base: Currency::EUR,
                              symbols: None,
                              date: None,
                          }),
                         ("http://api.fixer.io/2000-01-03?base=EUR",
                          Config {
                              base: Currency::EUR,
                              symbols: None,
                              date: Some(String::from("2000-01-03")),
                          }),
                         ("http://api.fixer.io/latest?base=EUR&symbols=AUD",
                          Config {
                              base: Currency::EUR,
                              symbols: Some(vec![Currency::AUD]),
                              date: None,
                          }),
                         ("http://api.fixer.io/latest?base=EUR&symbols=AUD,USD",
                          Config {
                              base: Currency::EUR,
                              symbols: Some(vec![Currency::AUD, Currency::USD]),
                              date: None,
                          })];


        for test in tests {
            assert_eq!(test.0, get_url(&test.1));
        }
    }
}