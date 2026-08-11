//! Public-API surface snapshots for the PARENT package (docs/public-api/).
//! Shared implementation + format docs: the `zenutils-apidoc` crate.
//!
//! heif-parse uses the default configuration, matching the pre-runner
//! snapshot test: supported surface = default features; features file = all
//! manifest features except `_*`-prefixed internal gates. The parent is a
//! single-package root, so discovery snapshots only heif-parse.
#[test]
fn public_api_surface_docs_are_current() {
    zenutils_apidoc::ApiDoc::new().workspace_dir("..").run();
}
