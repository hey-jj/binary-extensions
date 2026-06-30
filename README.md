# binary-extensions

Curated list of binary file extensions for text/binary classification.

This crate exposes one immutable list of 262 file extensions commonly
associated with binary files: archives, images, audio, video, office documents,
executables, fonts, RAW camera formats, and similar. It carries no logic and no
runtime dependencies.

Classification by extension is a heuristic. A few entries (for example `rtf`,
`m3u`, `slk`) are text-encoded formats kept for compatibility, so membership
does not prove a file holds binary bytes.

## Installation

```toml
[dependencies]
binary-extensions = "0.1"
```

## Usage

```rust
use binary_extensions::BINARY_EXTENSIONS;

assert!(BINARY_EXTENSIONS.contains(&"png"));
assert!(!BINARY_EXTENSIONS.contains(&"txt"));
```

`binary_extensions::all()` returns the same slice if you prefer a function call.

Entries are bare tokens without a leading dot, for example `"png"`, never
`".png"`. Strip the dot and lowercase a path extension before checking
membership, otherwise names like `IMAGE.PNG` are missed. Every entry is
lowercase except `"DS_Store"`, the macOS Finder metadata file, whose casing is
preserved and matched as is.

## License

Licensed under the [MIT license](LICENSE).
