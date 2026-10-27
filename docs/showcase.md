# Showcase

The checked-in render is generated from the same local mock-backed example shown
in the README. It demonstrates a typed `list_servers` call without a cloud
token or a request to Hetzner.

## Render corpus

| File | Contents | Reproduction |
|---|---|---|
| [`docs/showcase/server-inventory.svg`](showcase/server-inventory.svg) | Terminal-style view of the sample CLI output. | `docs/showcase/make-screenshots.sh` |

From the repository root, regenerate every render with:

```sh
docs/showcase/make-screenshots.sh
```

The script runs `cargo run --locked --quiet --example mock_cli`, captures its
output, and passes that output to the Rust SVG renderer:

```sh
cargo run --locked --quiet --example mock_cli > /tmp/hetzner-rs-showcase.txt
cargo run --locked --quiet --example render_showcase -- \
  /tmp/hetzner-rs-showcase.txt docs/showcase/server-inventory.svg
```

The renderer checks that the sample output still matches the captured lines so
the image cannot silently drift from the example. Its output is deterministic.
