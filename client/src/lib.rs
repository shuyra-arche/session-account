#![no_std]
//! Trusted Login requests construction under a configured account identity.
mod built;
mod client;
pub use built::Built;
pub use client::Client;
