# Mise Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/mise)](https://pkg.fluentci.io/mise)
[![ci](https://github.com/fluentci-io/mise-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/mise-plugin/actions/workflows/ci.yml)

A Fluent CI plugin to setup [mise](https://mise.jdx.dev).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm mise setup
```

## Functions

| Name    | Description                               |
| ------- | ----------------------------------------- |
| setup   | Download and install mise                 |
| install | Install a tool version [aliases: i]       |
| exec    | Execute a command with tool(s) set [aliases: x] |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/mise@v0.1.0?wasm=1", "setup", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: mise
    args: |
      setup
```
