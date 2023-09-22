# Mon-Mon
A monitor (service) for your monitor (display). Intercepts window behaviour when monitor configuration changes.

## Quick Start
1. `cp .env.example .env`
2. `cargo run`

## Installation
To make this app globally accessible, you can either drop the binary in your $PATH, or use `cargo`'s handy install command from the root of this project: `cargo install --path .`

## Running as Background Service
1. First make sure you have [installed](#installation) the `monmon` binary.
2. Determine the global installation path with `which monmon`
3. Run `monmon install [path-to-binary]`
_Note: the background service can be uninstalled with `monmon uninstall`_


## Troubleshooting
- "Command not found: monmon"
  - If you are using asdf package manager, try `asdf reshim`