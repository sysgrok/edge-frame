# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
* Breaking change: update to `yew 0.23`, `yewdux 0.13` and `yew-router 0.20`
* `kitchen-sink`: update Bulma from 0.9.3 to 1.0.4
* Bugfix: the "Save" button of `WifiSetup` dispatched `Some(conf)`, i.e. a message of type `Option<WifiConf>`, for which no dispatch is registered
* Bugfix: `RoleState::LoggedOut` was neither constructed nor matched anywhere
* `kitchen-sink`: the middleware for `RoleStore` now stands in for the device backend by confirming an authentication request with the `Admin` role and a logout request with the `None` role
* `kitchen-sink`: `index.html` now has a `<body>`, without which recent `trunk` versions refuse to build
* Update the remaining dependencies to their latest versions: `heapless 0.9`, `derive_more 2`, `strum` / `strum_macros 0.27`, `embedded-svc 0.29`, `gloo-net 0.7` and `embassy-sync 0.8`. `strum` is held at 0.27 rather than 0.28, because `embedded-svc 0.29` derives the iterator of its `AuthMethod` against 0.27
* Re-export the dependencies which are present in the public API - `anyhow`, `embassy-sync`, `embedded-svc`, `enumset`, `futures`, `gloo-net`, `log`, `num_enum`, `serde`, `strum`, `wasm-bindgen` and `web-sys` - as modules named after them, so that users do not have to track their exact versions. `yew`, `yew-router`, `yewdux` and `yewdux-middleware` are not re-exported, as their proc macros expand to absolute paths and thus need those crates as direct dependencies anyway

## [0.8.0] - 2024-02-01
* Breaking change: update to `yewdux 0.10` which introduces the notion of `Context`
* Breaking change: update to `heapless 0.8`
* Bugfix: Websocket messages were not deserialized properly
