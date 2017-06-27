use std::env;

pub struct Statics {
    pub TARGET: String,
    pub CARGO_PKG_AUTHORS: &'static str,
    pub CARGO_PKG_DESCRIPTION: &'static str,
    pub CARGO_PKG_HOMEPAGE: &'static str,
    pub CARGO_PKG_NAME: &'static str,
    pub CARGO_PKG_VERSION: &'static str
}

impl Statics {
    pub fn get() -> Statics {
        Statics {
            TARGET: env::var("TARGET").unwrap_or("Unable to retrive target-triple from the environment".to_owned()),
            CARGO_PKG_AUTHORS: env!("CARGO_PKG_AUTHORS"),
            CARGO_PKG_DESCRIPTION: env!("CARGO_PKG_DESCRIPTION"),
            CARGO_PKG_HOMEPAGE: env!("CARGO_PKG_HOMEPAGE"),
            CARGO_PKG_NAME: env!("CARGO_PKG_NAME"),
            CARGO_PKG_VERSION: env!("CARGO_PKG_VERSION")
        }
    }
}