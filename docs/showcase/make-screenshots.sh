#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"
mkdir -p docs/showcase
tmp_output="$(mktemp)"
trap 'rm -f "$tmp_output"' EXIT

cargo run --locked --quiet --example mock_cli > "$tmp_output"
cargo run --locked --quiet --example render_showcase -- \
  "$tmp_output" docs/showcase/server-inventory.svg
