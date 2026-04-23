arpbox
===========================

[<img alt="github" src="https://img.shields.io/badge/github-wcampbell0x2a/arpbox-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/wcampbell0x2a/arpbox)
[<img alt="crates.io" src="https://img.shields.io/crates/v/arpbox.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/arpbox)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-arpbox-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/arpbox)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/wcampbell0x2a/arpbox/main.yml?branch=master&style=for-the-badge" height="20">](https://github.com/wcampbell0x2a/arpbox/actions?query=branch%3Amaster)

Enrich ARP data with device information from NetBox

## Build arpbox
Either build from published source in crates.io.
```
$ cargo install arpbox --locked
```

Or download from [github releases](https://github.com/wcampbell0x2a/arpbox/releases).

## Usage

### Configure

Create `~/.config/arpbox/config.toml`.

#### v1 (Token authentication)
```toml
netbox_url = "https://netbox.example.com"
netbox_token = "your_api_token"
```

This uses the `Token <token>` authorization header.

#### v2 (Bearer/key authentication)
```toml
netbox_url = "https://netbox.example.com"
netbox_token = "your_api_token"
netbox_key = "your_key_id"
```

This uses the `Bearer nbt_<key>.<token>` authorization header.

### Run

Pipe arp-scan into arpbox.
```console
$ arp-scan --localnet | arpbox
```

Lines with a NetBox match are enriched:
```
192.168.1.1	00:11:22:33:44:55	(netbox://device-name:asset-tag:interface)	Vendor Name
```

Lines without a match pass through unchanged.
