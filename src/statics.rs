use std::env;

pub struct Statics {
    pub target: String,
    pub cargp_pkg_authors: &'static str,
    pub cargo_pkd_description: &'static str,
    pub cargo_pkd_homepage: &'static str,
    pub cargo_pkg_name: &'static str,
    pub cargo_pkg_version: &'static str
}

impl Statics {
    pub fn get() -> Statics {
        Statics {
            target: env::var("TARGET").unwrap_or_else(|_| "Unable to retrive target-triple from the environment".to_owned()),
            cargp_pkg_authors: env!("CARGO_PKG_AUTHORS"),
            cargo_pkd_description: env!("CARGO_PKG_DESCRIPTION"),
            cargo_pkd_homepage: env!("CARGO_PKG_HOMEPAGE"),
            cargo_pkg_name: env!("CARGO_PKG_NAME"),
            cargo_pkg_version: env!("CARGO_PKG_VERSION")
        }
    }
}