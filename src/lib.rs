/*!
This crate provides both the library, and command-line tool for 'mcfg': Machine configurator,
a cross-platform meta-package manager.

The library consists of two parts; the [`actions`](actions/index.html) module implements a simple
framework for adding commands to the command-line interface, and the [`shared`](actions/shared.html)
module provides the common models and other utilities required by the actions.

More information is found in the [User Guide](https://simonkjohnston.life/rust-mcfg/).
*/

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate prettytable;

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

/// The reported name for the command-line application.
pub const APP_NAME: &str = "mcfg";

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[macro_use]
pub mod reporter;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod actions;

pub mod error;

pub mod shared;
