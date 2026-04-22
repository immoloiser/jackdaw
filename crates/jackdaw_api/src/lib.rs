//! Public API for Jackdaw editor extensions.
//!
//! This crate is a thin facade over [`jackdaw_api_internal`]. It exposes
//! exactly the surface that third-party extension and game authors are
//! expected to use. Internal plumbing (the FFI entry-point structs used
//! by `export_extension!` / `export_game!`, registries used by the
//! editor binary, macro-emission helpers) lives behind
//! `jackdaw_api_internal` and is not re-exported here.
//!
//! # For static consumers
//!
//! ```toml
//! jackdaw_api = "0.4"
//! ```
//!
//! # For dylib extensions
//!
//! ```toml
//! jackdaw_api = { version = "0.4", features = ["dynamic_linking"] }
//! bevy = "0.18"  # `dynamic_linking` is pulled in transitively
//! ```
//!
//! Matching the `dynamic_linking` feature on the host binary
//! (`jackdaw`'s `dylib` feature) is mandatory for runtime dylib
//! loading to be sound.

// Force a link dependency on `jackdaw_dylib` so the compiled jackdaw
// types live in a single shared `.so` that both the editor binary
// and every extension dylib see. Mirrors the `bevy_dylib` trick that
// `bevy/dynamic_linking` uses.
#[cfg(feature = "dynamic_linking")]
#[allow(unused_imports)]
use jackdaw_dylib as _;

// Curated public surface. Pick each item individually rather than
// `pub use jackdaw_api_internal::*;` so internal plumbing (the `ffi`
// module, the `export` macro helpers, `registries`) stays out of the
// extension-author-facing docs.

// Prelude extension authors import with `use jackdaw_api::prelude::*;`.
pub use jackdaw_api_internal::prelude;

// Entry-point macros for dylib extensions and games. Invoked at most
// once per dylib crate; the generated `extern "C"` function is what
// the loader's `dlopen` looks up.
pub use jackdaw_api_internal::{export_extension, export_game};

// Top-level traits and types authors subclass / reference directly.
pub use jackdaw_api_internal::{
    DynJackdawExtension, ExtensionContext, ExtensionPoint, HierarchyWindow, InspectorWindow,
    JackdawExtension, MenuEntryDescriptor, PanelContext, SectionBuildFn, WindowDescriptor,
};

// The runtime plugin authors add when embedding jackdaw.
pub use jackdaw_api_internal::ExtensionLoaderPlugin;

// Module-level re-exports: each is a curated public surface of its
// own. The internal crate has additional items (StoredExtension,
// ExtensionResourceOf, cleanup fns, CatalogEntry, ...) that stay
// behind `jackdaw_api_internal` for the editor binary to consume.
pub use jackdaw_api_internal::{lifecycle, operator, pie, runtime, snapshot};

// `macros` re-export keeps the proc-macro crate discoverable by the
// derive macro's `$crate` paths. Authors can also use the
// `operator` attribute macro directly from `jackdaw_api::prelude`.
pub use jackdaw_api_internal::macros;

// Scene AST types for extensions that read or write scene files.
pub use jackdaw_api_internal::jsn;
