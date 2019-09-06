# hApp Example

This simple Rust hApp demonstrates how to use Nix derivations to:
- Build the hApp DNA locally, or via use of the hApp's Nix expression in another project
- Develop using local development branches of dependencies such as holochain-rust
- Integrate with Hydra, CircleCI and other CI services

## Nix Configuration

To build a hApp using Nix and the `https://github.com/Holo-Host/holo-nixpkgs` tools, copy the
`default.nix`, `pkgs.nix` and `shell.nix` files here, with your hApp's name substituted for the
`happ-example` name in the files.  This enables you to build your hApp's DNA using tooling supplied
by `holo-nixpkgs`.

### Build release DNA

Run `nix-build -A happ-example`.

#### Develop against a local `holochain-rust` version

Run `nix-shell -I holochain-rust=../holochain-rust`, where `../holochain-rust`
is a path to `holochain-rust` checkout. Then, develop as usual.

#### Develop against built-in `holochain-rust` version

See previous section, run `nix-shell` without arguments.

## Testing

Holochain hApps typically use both Rust unit tests, and Javascript-based "Scenario" tests written
using the Try-o-rama `npm` module.

### Binary Caches

Both testing and production builds requires access to significant prerequises.  Instead of
rebuilding these, we can access the cached binary assets available from both `nixos.org` and
`holo.host`.

To accelerate building of the `holochain-rust` and other dependencies, configure the appropriate
binary caches.

#### Configure `nix.conf`

To take leverage of Holo and NixOS binary caches, set `substituters` and
`trusted-public-keys` in `nix.conf` to the following values:

```
substituters = https://cache.holo.host/ https://cache.nixos.org/
trusted-public-keys = cache.holo.host-1:lNXIXtJgS9Iuw4Cu6X0HINLu9sTfcjEntnrgwMQIMcE= cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY=
```

For single-user installs (`nix-shell -p nix-info --run nix-info` prints
`multi-user: no`), config file is in `~/.config/nix/nix.conf`.

Otherwise, for multi-user installs, config file is in `/etc/nix/nix.conf` and
changing it requires root access.

### Scenario Testing

Create the Holochain scenario tests in the `test/` directory.  These can be manually executed using
`make test` (remember to enter `nix-shell` first, or hit `make nix-test`.)  The `hc test` utility
will be used to execute the tests.

#### CircleCI

[`.circleci/config.yml`](.circleci/config.yml) is used to execute `hc test`.
The `cache.holo.host` and `cache.nixos.org` binary caches are used to
accelerate the process of obtaining the build artifacts.

Holochain hApps typically use Javascript-based "Scenario" tests written using Try-o-rama.  

#### Hydra

If the `test/` directory exists, it is assumed to contain a `package-lock.json` specifying all of
the required NodeJS `npm` artifacts, and an `index.js` containing the Scenario tests to run.  If
found, the underlying `holo-nixpkgs` Nix configuration will direct the Hydra server to execute the
tests.
