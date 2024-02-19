//! Main entry point for CosmosCli

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use cosmos_cli::application::APP;

/// Boot CosmosCli
fn main() {
    abscissa_core::boot(&APP);
}
