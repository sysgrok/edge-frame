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

## [0.8.0] - 2024-02-01
* Breaking change: update to `yewdux 0.10` which introduces the notion of `Context`
* Breaking change: update to `heapless 0.8`
* Bugfix: Websocket messages were not deserialized properly
