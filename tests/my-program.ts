import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyProgram } from "../target/types/my_program";
import { assert } from "chai";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

describe("my-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.MyProgram as Program<MyProgram>;
  const myCounter = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("my-counter"),
      provider.wallet.publicKey.toBuffer(),
    ],
    program.programId
  )[0];

  it("Creates and initializes my counter", async () => {
    const txid = await program.methods
      .initialize(new anchor.BN(1234))
      .accounts({
        myCounter: myCounter,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Initialize tx:", txid);
  });

  it("Add an even number to my counter", async () => {
    const txid = await program.methods
      .addEven(new anchor.BN(420))
      .accounts({
        myCounter: myCounter,
        user: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Add even tx:", txid);
  });

  it("Minus an odd number to my counter", async () => {
    const txid = await program.methods
      .minusOdd(new anchor.BN(69))
      .accounts({
        myCounter: myCounter,
        user: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Minus odd tx:", txid);
  });

  it("Close my counter account", async () => {
    const txid = await program.methods
      .close()
      .accounts({
        myCounter: myCounter,
        user: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Close counter tx:", txid);
  });
});
