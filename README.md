# Scanbo Relay Chain

A custom relay chain built on the Polkadot SDK, based on the Westend runtime template.

## 🚀 Setup & Build

### Prerequisites
- [Rust & Cargo](https://rustup.rs/)
- Dependencies for Polkadot SDK (see [Substrate docs](https://docs.substrate.io/install/))

### Build Instructions
Build the node and parachain workers in release mode:
```bash
cargo build --release --bin polkadot \
  --bin polkadot-execute-worker \
  --bin polkadot-prepare-worker \
  --features scanbo-relay-native
```

## 🛠 Chain Configuration

### 1. Clear Old Data (Optional)
```bash
rm -rf /tmp/alice /tmp/bob
```

### 2. Generate Raw Chain Spec
```bash
./target/release/polkadot build-spec --chain scanbo-relay-local --raw --disable-default-bootnode > scanbo-relay-local.json
```

## 🏃 Running the Chain

### Validator 1: Alice (Sudo)
```bash
./target/release/polkadot \
  --chain scanbo-relay-local.json \
  --alice \
  --validator \
  --base-path /tmp/alice \
  --port 30333 \
  --rpc-port 9944 \
  --unsafe-force-node-key-generation \
  --insecure-validator-i-know-what-i-do
```
*Take note of the Peer ID in the logs (e.g., `12D3KooWJooZQnES8eokLgZwYuA6RCFany8gMQxBRJq64WZ1QW1k`).*

### Validator 2: Bob
Replace `<ALICE_PEER_ID>` with the ID from above:
```bash
./target/release/polkadot \
  --chain scanbo-relay-local.json \
  --bob \
  --validator \
  --base-path /tmp/bob \
  --port 30334 \
  --rpc-port 9945 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<ALICE_PEER_ID> \
  --unsafe-force-node-key-generation \
  --insecure-validator-i-know-what-i-do
```

## 🆙 Runtime Upgrade

1. Increment `spec_version` in `polkadot/runtime/scanbo-relay/src/lib.rs`.
2. Rebuild the runtime:
   ```bash
   cargo build --release --bin polkadot --features scanbo-relay-native
   ```
3. Locate the WASM file:
   `./target/release/wbuild/scanbo-relay-runtime/scanbo_relay_runtime.compact.compressed.wasm`
4. Submit the upgrade extrinsic via Polkadot.js Apps:
   - **Sudo -> sudo(system.setCode(wasm_file))**

---

### Important Notes
- **macOS Users**: The `--insecure-validator-i-know-what-i-do` flag is required because the default secure validator mode is Linux-only.
- **Genesis**: Alice is configured as both a validator and the Sudo root account.

---

## 📂 Modifications Registry

Below is a list of the core files modified to implement the `scanbo-relay` chain:

### Runtime Layer
- **[polkadot/runtime/scanbo-relay/src/lib.rs](file:///Users/utsavbhikadiya2688/Code%20Projects/scanbo/newchain/polkadot-sdk/polkadot/runtime/scanbo-relay/src/lib.rs)**:
    - Renamed runtime to `scanbo-relay`.
    - Set `spec_version`, `impl_version`, and `transaction_version`.
    - Configured `RUNTIME_API_VERSIONS`.
- **[polkadot/runtime/scanbo-relay/src/genesis_config_presets.rs](file:///Users/utsavbhikadiya2688/Code%20Projects/scanbo/newchain/polkadot-sdk/polkadot/runtime/scanbo-relay/src/genesis_config_presets.rs)**:
    - Modified `westend_testnet_genesis` and `testnet_accounts`.
    - Unified validator and stash accounts (removed `//stash` derivation).
    - Ensured Alice is the Sudo key and a validator.
- **[polkadot/runtime/scanbo-relay/Cargo.toml](file:///Users/utsavbhikadiya2688/Code%20Projects/scanbo/newchain/polkadot-sdk/polkadot/runtime/scanbo-relay/Cargo.toml)**:
    - Renamed package to `scanbo-relay-runtime`.
    - Updated internal dependencies.

### Node & CLI Layer
- **[polkadot/node/service/src/chain_spec.rs](file:///Users/utsavbhikadiya2688/Code%20Projects/scanbo/newchain/polkadot-sdk/polkadot/node/service/src/chain_spec.rs)**:
    - Registered `scanbo_relay_local_testnet_config`.
    - Integrated `scanbo-relay-runtime` into the build process.
- **[polkadot/node/service/Cargo.toml](file:///Users/utsavbhikadiya2688/Code%20Projects/scanbo/newchain/polkadot-sdk/polkadot/node/service/Cargo.toml)**:
    - Added `scanbo-relay-runtime` dependency.
    - Defined `scanbo-relay-native` feature.
- **[polkadot/cli/Cargo.toml](file:///Users/utsavbhikadiya2688/Code%20Projects/scanbo/newchain/polkadot-sdk/polkadot/cli/Cargo.toml)**:
    - Added `scanbo-relay-native` feature to the CLI to enable the new runtime.
- **[Cargo.toml](file:///Users/utsavbhikadiya2688/Code%20Projects/scanbo/newchain/polkadot-sdk/Cargo.toml)** (Root):
    - Added `polkadot/runtime/scanbo-relay` to workspace members.
