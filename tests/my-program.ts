import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyProgram } from "../target/types/my_program";
import { assert } from "chai";

describe("my-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.MyProgram as Program<MyProgram>;
  const myCounter = anchor.web3.Keypair.generate();

  it("Creates and initializes my counter", async () => {
    await program.methods
      .initialize(new anchor.BN(1234))
      .accounts({
        myCounter: myCounter.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([myCounter])
      .rpc();

    const account = await program.account.myCounter.fetch(myCounter.publicKey);
    assert.ok(account.value.eq(new anchor.BN(1234)));
  });

  it("Add an even number to my counter", async () => {
    await program.methods
      .addEven(new anchor.BN(420))
      .accounts({
        myCounter: myCounter.publicKey,
      })
      .rpc();

    const account = await program.account.myCounter.fetch(myCounter.publicKey);
    assert.ok(account.value.eq(new anchor.BN(1654)));
  });

  it("Minus an odd number to my counter", async () => {
    await program.methods
      .minusOdd(new anchor.BN(69))
      .accounts({
        myCounter: myCounter.publicKey,
      })
      .rpc();

    const account = await program.account.myCounter.fetch(myCounter.publicKey);
    assert.ok(account.value.eq(new anchor.BN(1585)));
  });
  it("Get my counter", async () => {
    const account = await program.account.myCounter.fetch(myCounter.publicKey);
    assert.ok(account.value.eq(new anchor.BN(1585)));
  });
});
