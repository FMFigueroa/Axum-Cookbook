// region:    --- Modules

mod error;
pub mod mw_auth;
pub mod mw_res_map;
pub mod routes_login;
pub mod routes_static;

pub use self::error::{ClientError, Error, Result};

// endregion: --- Modules

pub const AUTH_TOKEN: &str = "auth-token";
