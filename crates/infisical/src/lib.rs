#![feature(stmt_expr_attributes)]

pub mod api;
pub use api::universal_auth_login;

pub mod cache;
pub mod constants;

pub mod auth;
pub mod client;
pub mod error;
pub use client::Client;
pub mod manager;

pub(crate) mod helper;
