# Squareknot (early stages)
A graph library focusing on ergonomics and flexibility via zero cost abstractions.

## Usage

To use the library, first find a commit you wish to use, then add the following to your `Cargo.toml`, replacing "X" the commit id so that the same version is being referenced as the library updates.

Alternatively the library may be forked.

```toml
[dependencies]
squareknot = { git = https://github.com/AarSeBail/squareknot.git, commit = "X" }
```

## Documentation

Documentation may be viewed with ``cargo doc --workspace --exclude benches --open``.

## Show me around

The codebase is modular, with features being split among the crates in `crates/`.

The main graph types are contained in `squareknot_graph`. For more details, see the documentation.
