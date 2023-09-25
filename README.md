# Mon-Mon
A monitor (service) for your monitor (display).

Listens for changes to display configuration (e.g. plugging in an additional screen) and allows you to blacklist monitors from displaying windows when they are initially connected - useful for public settings where you don't want to flood the audience with all of your open applications.

## Quick Start
1. `cp .env.example .env`
2. `cargo run`

## Installation
To make this app globally accessible, you can either drop the binary in your $PATH, or use `cargo`'s handy install command from the root of this project: `cargo install --path .`

You will then be able to run commands from your terminal from any directory, e.g. `$ monmon watch`

## Commands
- `monmon` - the default command; shoves everything to your primary display and exits.
- `monmon watch` - runs continually; will only move windows to the primary display if it detects a blacklisted monitor being plugged in (see [quick start](#quick-start))

## Running as Background Service
1. First make sure you have [installed](#installation) the `monmon` binary.
2. Determine the global installation path with `which monmon`
3. Run `BLACKLIST_DISPLAYS="[comma-separated-displays]" monmon install [path-to-binary]`
_Note: the background service can be uninstalled with `monmon uninstall`_


## Troubleshooting
- "Command not found: monmon"
  - If you are using asdf package manager, try `asdf reshim`