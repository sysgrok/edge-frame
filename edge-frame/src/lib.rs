#![allow(clippy::let_unit_value)]
#![allow(async_fn_in_trait)]
#![cfg_attr(
    any(feature = "assets-serve", all(feature = "dto", not(feature = "web"))),
    no_std
)]
#![cfg_attr(feature = "web", recursion_limit = "1024")]

#[cfg(any(
    all(feature = "assets-prepare", feature = "assets-serve"),
    all(feature = "assets-prepare", feature = "web"),
    all(feature = "assets-prepare", feature = "dto")
))]
compile_error!(
    "Feature `assets-prepare` is not compatible with features `assets-serve`, `web` and `dto`."
);

#[cfg(all(feature = "assets-serve", feature = "web"))]
compile_error!("Feature `assets-serve` is not compatible with feature `web`.");

// The dependencies which are present in the public API of `edge-frame`, re-exported so that
// the users of the crate do not have to track their exact versions on their own.
//
// `yew`, `yew-router`, `yewdux` and `yewdux-middleware` are deliberately not re-exported, as
// their proc macros expand to absolute `::yew` / `::yewdux` paths, which only resolve for a
// crate in the extern prelude. Users therefore have to depend on those directly anyway.

#[cfg(feature = "anyhow")]
pub mod anyhow {
    pub use ::anyhow::*;
}

#[cfg(feature = "embassy-sync")]
pub mod embassy_sync {
    pub use ::embassy_sync::*;
}

#[cfg(feature = "embedded-svc")]
pub mod embedded_svc {
    pub use ::embedded_svc::*;
}

#[cfg(feature = "enumset")]
pub mod enumset {
    pub use ::enumset::*;
}

#[cfg(feature = "futures")]
pub mod futures {
    pub use ::futures::*;
}

#[cfg(feature = "gloo-net")]
pub mod gloo_net {
    pub use ::gloo_net::*;
}

#[cfg(feature = "log")]
pub mod log {
    pub use ::log::*;
}

#[cfg(feature = "num_enum")]
pub mod num_enum {
    pub use ::num_enum::*;
}

#[cfg(feature = "serde")]
pub mod serde {
    pub use ::serde::*;
}

#[cfg(feature = "strum")]
pub mod strum {
    pub use ::strum::*;
}

#[cfg(feature = "wasm-bindgen")]
pub mod wasm_bindgen {
    pub use ::wasm_bindgen::*;
}

#[cfg(feature = "web-sys")]
pub mod web_sys {
    pub use ::web_sys::*;
}

#[cfg(feature = "web")]
pub use web::*;

#[cfg(feature = "web")]
#[path = "."]
mod web {
    pub mod auth;
    pub mod field;
    pub mod frame;
    pub mod ipv4;
    pub mod loading;
    pub mod middleware;
    pub mod navbar;
    pub mod role;
    pub mod util;
    pub mod wifi;
    pub mod wifi_setup;
}

#[cfg(any(feature = "assets-serve", feature = "assets-prepare"))]
pub mod assets;

#[cfg(feature = "dto")]
pub mod dto;
