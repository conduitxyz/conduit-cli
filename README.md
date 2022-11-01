## Conduit

![Github Actions][gha-badge] [![Telegram Chat][tg-badge]][tg-url] [![Telegram Support][tg-support-badge]][tg-support-url]

[gha-badge]: https://img.shields.io/github/workflow/status/conduit-xyz/conduit/test?style=flat-square
[tg-support-url]: https://t.me/conduitsupport

**Conduit is the cli for the Conduit app.**

## Installation

First run the command below to get `conduitup`, the Conduit toolchain installer:

```sh
curl -L https://conduit-cli.conduit.xyz | bash
```

If you do not want to use the redirect, feel free to manually download the conduitup installation script from [here](https://raw.githubusercontent.com/conduit-xyz/conduit-cli/master/conduitup/install).

Then, run `conduitup` in a new terminal session or after reloading your `PATH`.

Other ways to use `conduitup`, and other documentation, can be found [here](./conduitup). Happy conduiting!

### Installing from Source

For people that want to install from source, you can do so like below:

```sh
git clone https://github.com/conduit-xyz/conduit-cli
cd conduit
# conduit
cargo install --path ./cli --profile local --bins --locked --force
```

Or via `cargo install --git https://github.com/conduit-xyz/conduit-cli --profile local --locked conduit`.

### Installing for CI in Github Action

See [https://github.com/conduit-xyz/conduit-toolchain](https://github.com/conduit-xyz/conduit-toolchain) GitHub Action.

### Installing via Docker

Conduit maintains a [Docker image repository](https://github.com/conduit-xyz/conduit/pkgs/container/conduit).

You can pull the latest release image like so:

```sh
docker pull ghcr.io/conduit-xyz/conduit-cli:latest
```

### Manual Download

You can manually download nightly releases [here](https://github.com/conduit-xyz/conduit-cli/releases).