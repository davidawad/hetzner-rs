use std::{env, fs, path::PathBuf};

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args_os().skip(1);
    let input = PathBuf::from(args.next().ok_or("missing CLI output path")?);
    let output = PathBuf::from(args.next().ok_or("missing SVG output path")?);
    if args.next().is_some() {
        return Err("expected CLI output path and SVG output path".into());
    }

    let sample = fs::read_to_string(input)?;
    let lines = sample.lines().collect::<Vec<_>>();
    if lines != ["Servers: 0", "Source: local mock"] {
        return Err("mock_cli output changed; update the showcase renderer".into());
    }

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="960" height="540" viewBox="0 0 960 540">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1"><stop stop-color="#0b1220"/><stop offset="1" stop-color="#172554"/></linearGradient>
    <filter id="shadow" x="-20%" y="-20%" width="140%" height="150%"><feDropShadow dx="0" dy="18" stdDeviation="20" flood-color="#020617" flood-opacity=".45"/></filter>
  </defs>
  <rect width="960" height="540" rx="28" fill="url(#bg)"/>
  <circle cx="810" cy="110" r="140" fill="#2563eb" opacity=".12"/>
  <text x="68" y="85" fill="#93c5fd" font-family="Inter, ui-sans-serif, sans-serif" font-size="16" font-weight="700" letter-spacing="2">HETZNER CLOUD · RUST CLIENT</text>
  <text x="68" y="137" fill="#f8fafc" font-family="Inter, ui-sans-serif, sans-serif" font-size="32" font-weight="700">A safe first API call</text>
  <text x="68" y="169" fill="#94a3b8" font-family="Inter, ui-sans-serif, sans-serif" font-size="17">Typed client request, served entirely by a local mock.</text>
  <g filter="url(#shadow)">
    <rect x="68" y="218" width="824" height="246" rx="18" fill="#080d18" stroke="#334155"/>
    <path d="M68 236a18 18 0 0 1 18-18h788a18 18 0 0 1 18 18v30H68z" fill="#111827"/>
    <circle cx="94" cy="242" r="6" fill="#f87171"/><circle cx="116" cy="242" r="6" fill="#fbbf24"/><circle cx="138" cy="242" r="6" fill="#4ade80"/>
    <text x="480" y="247" text-anchor="middle" fill="#64748b" font-family="ui-monospace, SFMono-Regular, monospace" font-size="13">mock_cli</text>
    <text x="102" y="316" fill="#60a5fa" font-family="ui-monospace, SFMono-Regular, monospace" font-size="19">$ cargo run --example mock_cli</text>
    <text x="102" y="367" fill="#e2e8f0" font-family="ui-monospace, SFMono-Regular, monospace" font-size="20">{}</text>
    <text x="102" y="405" fill="#e2e8f0" font-family="ui-monospace, SFMono-Regular, monospace" font-size="20">{}</text>
    <text x="102" y="443" fill="#64748b" font-family="ui-monospace, SFMono-Regular, monospace" font-size="13">No token · no network request · repeatable sample</text>
  </g>
  <text x="68" y="503" fill="#64748b" font-family="Inter, ui-sans-serif, sans-serif" font-size="13">Generated from examples/mock_cli.rs output by examples/render_showcase.rs</text>
</svg>
"##,
        xml_escape(lines[0]),
        xml_escape(lines[1])
    );
    fs::write(output, svg)?;
    Ok(())
}
