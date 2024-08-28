# Stylus Call

The repository contains playground for testing calling external contracts in Stylus (`hotfix-v0.5.3`).

## Build

```sh
cargo stylus check --no-verify
```

## Export ABI

```sh
cargo stylus export-abi
```

## Deploy

```sh
cargo stylus deploy --private-key=<priv-key> -e=<rpc-endpoint>
```
