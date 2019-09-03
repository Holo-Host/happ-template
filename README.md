# hApp Example

This simple Rust hApp demonstrates how to use Nix derivations to:
- Build the hApp DNA locally, or via use of the hApp's Nix expression in another project
- Develop using local development branches of dependencies such as holochain-rust
- Integrate with Hydra, Circle-CI and other CI services

## Testing

Holochain hApps typically use Javascript-based "Scenario" tests written using Try-o-rama.  

## Build release DNA build

Run `nix-build -A example`.

### Develop against a local `holochain-rust` version

Run `nix-shell -I holochain-rust=../holochain-rust`, where `../holochain-rust`
is a path to `holochain-rust` checkout. Then, develop as usual.

### Develop against built-in `holochain-rust` version

See previous section, run `nix-shell` without arguments.
