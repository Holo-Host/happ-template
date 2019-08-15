## Build release DNA build

Run `nix-build -A example`.

## Develop against a local `holochain-rust` version

Run `nix-shell -I holochain-rust=../holochain-rust`, where `../holochain-rust`
is a path to `holochain-rust` checkout. Then, develop as usual.

## Develop against built-in `holochain-rust` version

See previous section, run `nix-shell` without arguments.
