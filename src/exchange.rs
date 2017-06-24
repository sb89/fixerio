/// The response from fixerio.
#[derive(Debug, Deserialize)]
pub struct Exchange {
    /// The base currency requested.
    pub base: String,
    /// The date for which the exchange rates are for.
    pub date: String,
    /// The exhcange rates for the base currency.
    pub rates: Rates,
}

/// The exchange rates for the base currency within the response.
#[derive(Debug, Deserialize)]
pub struct Rates {
    /// Australian dollar
    #[serde(rename = "AUD", default)]
    pub aud: f32,
    /// Bulgarian lev
    #[serde(rename = "BGN", default)]
    pub bgn: f32,
    /// Brazilian real
    #[serde(rename = "BRL", default)]
    pub brl: f32,
    #[serde(rename = "CAD", default)]
    /// Canadian dollar
    pub cad: f32,
    #[serde(rename = "CHF", default)]
    /// Swiss franc
    pub chf: f32,
    /// Chinese yuan
    #[serde(rename = "CNY", default)]
    pub cny: f32,
    /// Czech koruna
    #[serde(rename = "CZK", default)]
    pub czk: f32,
    /// Danish krone
    #[serde(rename = "DKK", default)]
    pub dkk: f32,
    /// Euro
    #[serde(rename = "EUR", default)]
    pub eur: f32,
    /// Pound sterling
    #[serde(rename = "GBP", default)]
    pub gbp: f32,
    /// Hong Kong dollar
    #[serde(rename = "HKD", default)]
    pub hkd: f32,
    /// Croatian kuna
    #[serde(rename = "HRK", default)]
    pub hrk: f32,
    /// Hungarian forint
    #[serde(rename = "HUF", default)]
    pub huf: f32,
    /// Indonesian rupiah
    #[serde(rename = "IDR", default)]
    pub idr: f32,
    /// Israeli new shekel
    #[serde(rename = "ILS", default)]
    pub ils: f32,
    /// Indian rupee
    #[serde(rename = "INR", default)]
    pub inr: f32,
    /// Japanese yen
    #[serde(rename = "JPY", default)]
    pub jpy: f32,
    /// South Korean won
    #[serde(rename = "KRW", default)]
    pub krw: f32,
    /// Mexican peso
    #[serde(rename = "MXN", default)]
    pub mxn: f32,
    /// Malasyian ringgit
    #[serde(rename = "MYR", default)]
    pub myr: f32,
    /// Norwegian krone
    #[serde(rename = "NOK", default)]
    pub nok: f32,
    /// New Zealand dollar
    #[serde(rename = "NZD", default)]
    pub nzd: f32, 
    /// Philippine peso
    #[serde(rename = "PHP", default)]
    pub php: f32,
    /// Polish złoty
    #[serde(rename = "PLN", default)]
    pub pln: f32,
    /// Romanian leu
    #[serde(rename = "RON", default)]
    pub ron: f32,
    /// Russian ruble
    #[serde(rename = "RUB", default)]
    pub rub: f32,
    /// Swedish krona
    #[serde(rename = "SEK", default)]
    pub sek: f32,
    /// Singapore dollar
    #[serde(rename = "SGD", default)]
    pub sgd: f32,
    /// Thai baht
    #[serde(rename = "THB", default)]
    pub thb: f32,
    /// Turkish lira
    #[serde(rename = "TRY", default)]
    pub try: f32,
    /// United States dollar
    #[serde(rename = "USD", default)]
    pub usd: f32,
    /// South African rand
    #[serde(rename = "ZAR", default)]
    pub zar: f32
}

/// Fixerio currency.
#[derive(Debug, PartialEq)]
pub enum Currency {
    /// Australian dollar
    AUD,
    /// Bulgarian lev
    BGN,
    /// Brazilian real
    BRL,
    /// Canadian dollar
    CAD,
    /// Swiss franc
    CHF,
    /// Chinese yuan
    CNY,
    /// Czech koruna
    CZK,
    /// Danish krone
    DKK,
    /// Euro
    EUR,
    /// Pound sterling
    GBP,
    /// Hong Kong dollar
    HKD,
    /// Croatian kuna
    HRK,
    /// Hungarian forint
    HUF,
    /// Indonesian rupiah
    IDR,
    /// Israeli new shekel
    ILS,
    /// Indian rupee
    INR,
    /// Japanese yen
    JPY,
    /// South Korean won
    KRW,
    /// Mexican peso
    MXN,
    /// Malasyian ringgit
    MYR,
    /// Norwegian krone
    NOK,
    /// New Zealand dollar
    NZD,
    /// Philippine peso
    PHP,
    /// Polish złoty
    PLN,
    /// Romanian leu
    RON,
    /// Russian ruble
    RUB,
    /// Swedish krona
    SEK,
    /// Singapore dollar
    SGD,
    /// Thai baht
    THB,
    /// Turkish lira
    TRY,
    /// United States dollar
    USD,
    /// South African rand
    ZAR,
}

impl Currency {
    /// Return the string representation.
    pub fn string(&self) -> &str {
        match *self {
            Currency::AUD => "AUD",
            Currency::BGN => "BGN",
            Currency::BRL => "BRL",
            Currency::CAD => "CAD",
            Currency::CHF => "CHF",
            Currency::CNY => "CNY",
            Currency::CZK => "CZK",
            Currency::DKK => "DKK",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::HKD => "HKD",
            Currency::HRK => "HRK",
            Currency::HUF => "HUF",
            Currency::IDR => "IDR",
            Currency::ILS => "ILS",
            Currency::INR => "INR",
            Currency::JPY => "JPY",
            Currency::KRW => "KRW",
            Currency::MXN => "MXN",
            Currency::MYR => "MYR",
            Currency::NOK => "NOK",
            Currency::NZD => "NZD",
            Currency::PHP => "PHP",
            Currency::PLN => "PLN",
            Currency::RON => "RON",
            Currency::RUB => "RUB",
            Currency::SEK => "SEK",
            Currency::SGD => "SGD",
            Currency::THB => "THB",
            Currency::TRY => "TRY",
            Currency::USD => "USD",
            Currency::ZAR => "ZAR"
        }
    }
}