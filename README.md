# Basic Solana Developing Process with Anchor Framework (v0.29.0)

Full tutorial:
https://emersonliuuu.notion.site/Basic-Solana-Developing-Process-with-Anchor-Framework-v0-29-0-527e8d64db4c4b4e8449e76e12e437a1?pvs=4

Run the following commands to start the project:

```bash
git clone git@github.com:emersonliuuu/counter-program.git

cd counter-program
git submodule update --init

anchor keys list
```

Replace the key with your own key in

1. program/src/lib.rs `declare_id!("------Your key here------");`
2. Anchor.toml
   `[programs.localnet]
my_program = "------Your key here------"`

Then run the following commands:

```bash
anchor test
```

NOTICE:
`anchor test` will build, deploy and run the all tests under `tests` folder. You can change the cluster in `Anchor.toml` to test on different clusters, e.g. `localnet`, `devnet`, `testnet`.

Start the frontend

```bash
cd app
yarn
yarn dev
# open localhost:3000
```
