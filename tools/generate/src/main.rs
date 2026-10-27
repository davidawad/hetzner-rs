//! Regenerate `spec/cloud.openapi-3.0.json` and `src/client.rs` from the
//! vendored OpenAPI 3.1 snapshot in `spec/cloud.spec.json`.

mod convert;

use std::{fs, path::Path, path::PathBuf, process::Command};

use openapiv3::OpenAPI;
use progenitor::{GenerationSettings, Generator};
use serde_json::Value;

const SOURCE: &str = "spec/cloud.spec.json";
const CONVERTED: &str = "spec/cloud.openapi-3.0.json";
const CLIENT: &str = "src/client.rs";

/// rustfmt needs two passes to settle prettyplease's block doc comments.
const MAX_RUSTFMT_PASSES: usize = 4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;

    let mut document: Value = serde_json::from_slice(&fs::read(root.join(SOURCE))?)?;
    let summary = convert::downconvert(&mut document)?;
    let mut converted = serde_json::to_string_pretty(&document)?;
    converted.push('\n');
    fs::write(root.join(CONVERTED), &converted)?;
    println!(
        "wrote {CONVERTED}: {} operations, {} paths",
        summary.operations, summary.paths
    );

    let spec: OpenAPI = serde_json::from_str(&converted)?;
    let mut generator = Generator::new(&GenerationSettings::default());
    let tokens = generator.generate_tokens(&spec)?;
    let syntax = syn::parse2(tokens)?;
    let client = root.join(CLIENT);
    fs::write(&client, prettyplease::unparse(&syntax))?;
    let passes = rustfmt_until_stable(&client)?;
    println!("wrote {CLIENT} (rustfmt passes: {passes})");
    Ok(())
}

fn rustfmt_until_stable(path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let mut previous = fs::read(path)?;
    for pass in 1..=MAX_RUSTFMT_PASSES {
        let status = Command::new("rustfmt")
            .args(["--edition", "2021"])
            .arg(path)
            .status()?;
        if !status.success() {
            return Err(format!("rustfmt failed on {}: {status}", path.display()).into());
        }
        let current = fs::read(path)?;
        if current == previous {
            return Ok(pass);
        }
        previous = current;
    }
    Err(format!(
        "rustfmt did not converge on {} after {MAX_RUSTFMT_PASSES} passes",
        path.display()
    )
    .into())
}
