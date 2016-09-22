//! `conductor` as a reusable API, so that you can call it from other tools.

// Enable clippy if our Cargo.toml file asked us to do so.
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

// Enable as many useful Rust and Clippy warnings as we can stand.  We'd
// also enable `trivial_casts`, but we're waiting for
// https://github.com/rust-lang/rust/issues/23416.
#![warn(missing_copy_implementations,
        missing_debug_implementations,
        missing_docs,
        trivial_numeric_casts,
        unsafe_code,
        unused_extern_crates,
        unused_import_braces,
        unused_qualifications)]
#![cfg_attr(feature="clippy", warn(cast_possible_truncation))]
#![cfg_attr(feature="clippy", warn(cast_possible_wrap))]
#![cfg_attr(feature="clippy", warn(cast_precision_loss))]
#![cfg_attr(feature="clippy", warn(cast_sign_loss))]
#![cfg_attr(feature="clippy", warn(missing_docs_in_private_items))]
#![cfg_attr(feature="clippy", warn(mut_mut))]
// We allow `println!` only in the `cmd` submodule.  If you want to print
// debugging output, using `debug!`, or have `cmd` pass you an `io::Write`
// implementation.
#![cfg_attr(feature="clippy", warn(print_stdout))]
// This allows us to use `unwrap` on `Option` values (because doing makes
// working with Regex matches much nicer) and when compiling in test mode
// (because using it in tests is idiomatic).
#![cfg_attr(all(not(test), feature="clippy"), warn(result_unwrap_used))]
#![cfg_attr(feature="clippy", warn(unseparated_literal_suffix))]
#![cfg_attr(feature="clippy", warn(wrong_pub_self_convention))]

// Fail hard on warnings.  This will be automatically disabled when we're
// used as a dependency by other crates, thanks to Cargo magic.
#![deny(warnings)]

extern crate docker_compose;
#[cfg(test)]
extern crate env_logger;
extern crate glob;
extern crate handlebars;
extern crate includedir;
#[macro_use]
extern crate log;
extern crate phf;
#[cfg(test)]
extern crate rand;
extern crate retry;
extern crate rustc_serialize;
extern crate shlex;
extern crate url;

pub use util::{Error, err};
pub use default_tags::DefaultTags;
pub use ovr::Override;
pub use project::{Project, Pods, Overrides};
pub use pod::{Pod, OverrideFiles, AllFiles};
pub use repos::{Repos, Repo};
pub use repos::Iter as RepoIter;

#[macro_use]
mod util;
#[macro_use]
pub mod command_runner;
pub mod cmd;
mod default_tags;
pub mod dir;
pub mod exec;
mod ext;
mod ovr;
mod pod;
mod project;
mod repos;
mod template;

/// Include raw data files into our binary at compile time using the
/// `includedir_codegen` and `includedir` crates.  The actual code
/// generation is performed by our top-level `build.rs` script.
mod data {
    include!(concat!(env!("OUT_DIR"), "/data.rs"));
}
