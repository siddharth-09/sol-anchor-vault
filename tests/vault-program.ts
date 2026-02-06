import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VaultProgram } from "../target/types/vault_program";
import { assert } from "chai";

describe("Vault Program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider)

  const program = anchor.workspace.vaultProgram as Program<VaultProgram>;
  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();
  const anatoly = anchor.web3.Keypair.generate();

  const getVaultPDA = (vaultAuthority: anchor.web3.PublicKey) => {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), vaultAuthority.toBuffer()],
      program.programId
    );
  };
  
  const [vaultAlicePDA] = getVaultPDA(alice.publicKey);
  const [vaultBobPDA] = getVaultPDA(bob.publicKey);

  it("1) Initialize Alice Vault!", async () => {

    await airdrop(provider.connection,alice.publicKey);

    const tx = await program.methods.initialize()
    .accountsStrict({
      user : alice.publicKey,
      vault : vaultAlicePDA,
      systemProgram : anchor.web3.SystemProgram.programId
    })
    .signers([alice])
    .rpc();

    const vaultData = await program.account.vaultAccount.fetch(vaultAlicePDA);
    assert.strictEqual(vaultData.vaultAuthority.toString(), alice.publicKey.toString(), "Vault authority should be Alice's public key");

    // console.log("Your transaction signature", tx);
  });

  it("2) Initialize Bob Vault!",async()=>{
    await airdrop(provider.connection,bob.publicKey);

    const tx = await program.methods.initialize().accounts({
      user : bob.publicKey,
      vault : vaultBobPDA,
      systemProgram : anchor.web3.SystemProgram.programId
    }).signers([bob]).rpc();
    const vaultData = await program.account.vaultAccount.fetch(vaultBobPDA);
    assert.strictEqual(vaultData.vaultAuthority.toString(),bob.publicKey.toString(),"Vault authority should be Bob's public key")
    // console.log("Your transaction signature ",tx);
  });
  it("3) Cannot initialize vault twice (Alice tries to initialize again)", async () => {
    let flag = "This should fail";
    try {
      await program.methods.initialize().accounts({
        user: alice.publicKey,
        vault: vaultAlicePDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([alice]).rpc();
    } catch (error) {
      flag = "Failed";
      // Should fail because account already exists
      assert.isTrue(error.toString().includes("already in use") || error.toString().includes("Error"), "Should fail with account already in use error");
    }
    assert.strictEqual(flag, "Failed", "Initializing vault twice should fail");
  });
  it("4) Cannot Initialize Vault for someone else",async()=>{
    let flag = "This should fail";
    try{
      //bob trying to initialize alice vault by signing it
      const tx = await program.methods.initialize().accounts({
        user : alice.publicKey,
        vault : vaultAlicePDA,
        systemProgram : anchor.web3.SystemProgram.programId
      }).signers([bob]).rpc()
    }catch(error){
      flag = "Failed";
      // console.log(error)
      assert.isTrue(error.toString().includes("Error") , "Should fail with signature mismatch error");
    }
    assert.strictEqual(flag , "Failed" , "Initializing vault for someone else should fail");
  });
  it("5) Alice Deposit to vault",async()=>{
    const amount = 10000000
    const vaultBalanceBefore = await provider.connection.getBalance(vaultAlicePDA);
    const userBalanceBefore = await provider.connection.getBalance(alice.publicKey);

    const tx = await program.methods.deposit(new anchor.BN(amount)).accountsStrict({
      user : alice.publicKey,
      vault : vaultAlicePDA,
      systemProgram : anchor.web3.SystemProgram.programId
    }).signers([alice]).rpc()
    const vaultBalanceAfter = await provider.connection.getBalance(vaultAlicePDA);
    const userBalanceAfter = await provider.connection.getBalance(alice.publicKey);

    assert.isTrue(vaultBalanceAfter > vaultBalanceBefore , "Vault balance should increase after deposit");
    assert.isTrue(userBalanceBefore > userBalanceAfter , "Vault balance should increase after deposit");

  });
  it("6) Bob Deposit to vault",async()=>{
    const amount = 10000000
    const vaultBalanceBefore = await provider.connection.getBalance(vaultBobPDA);
    const userBalanceBefore = await provider.connection.getBalance(bob.publicKey);

    const tx = await program.methods.deposit(new anchor.BN(amount)).accountsStrict({
      user : bob.publicKey,
      vault : vaultBobPDA,
      systemProgram : anchor.web3.SystemProgram.programId
    }).signers([bob]).rpc()
    const vaultBalanceAfter = await provider.connection.getBalance(vaultBobPDA);
    const userBalanceAfter = await provider.connection.getBalance(bob.publicKey);

    assert.isTrue(vaultBalanceAfter > vaultBalanceBefore , "Vault balance should increase after deposit");
    assert.isTrue(userBalanceBefore > userBalanceAfter , "Vault balance should increase after deposit");

  })
  it("7) Alice withdraw",async()=>{
    const vaultBalanceBefore = await provider.connection.getBalance(vaultAlicePDA);
    const userBalanceBefore = await provider.connection.getBalance(alice.publicKey);

    const tx = await program.methods.withdraw().accounts({
      user : alice.publicKey,
      vault : vaultAlicePDA,
      systemProgram : anchor.web3.SystemProgram.programId 
    }).signers([alice]).rpc()

    const vaultBalanceAfter = await provider.connection.getBalance(vaultAlicePDA);
    const userBalanceAfter = await provider.connection.getBalance(alice.publicKey);

    assert.isTrue(vaultBalanceAfter < vaultBalanceBefore,"Vault balance should decrease from this")
    assert.isTrue(userBalanceBefore < userBalanceAfter,"user balance should Increase from this")

  });
  it("8) Bob withdraw",async()=>{
    const vaultBalanceBefore = await provider.connection.getBalance(vaultBobPDA);
    const userBalanceBefore = await provider.connection.getBalance(bob.publicKey);

    const tx = await program.methods.withdraw().accounts({
      user : bob.publicKey,
      vault : vaultBobPDA,
      systemProgram : anchor.web3.SystemProgram.programId 
    }).signers([bob]).rpc()

    const vaultBalanceAfter = await provider.connection.getBalance(vaultAlicePDA);
    const userBalanceAfter = await provider.connection.getBalance(alice.publicKey);
    
    assert.isTrue(vaultBalanceAfter < vaultBalanceBefore,"Vault balance should decrease from this")
    assert.isTrue(userBalanceBefore < userBalanceAfter,"user balance should Increase from this")

  });
  it("9) Bob Cannot withdraw From Alice Vault",async()=>{
    let flag = "This should fail";
    try{
      const tx = await program.methods.withdraw().accounts({
        user : bob.publicKey,
        vault : vaultAlicePDA,
        systemProgram : anchor.web3.SystemProgram.programId
      }).signers([bob]).rpc();
    }catch(error){
      flag = "Failed"
    }
    assert.strictEqual(flag ,"Failed","This should fail cause mismatch signature")
  });
  it("10) Close Alice Vault",async()=>{
    try{
      const tx = await program.methods.close().accounts({
        user : alice.publicKey,
        vault : vaultAlicePDA,
        systemProgram : anchor.web3.SystemProgram.programId
      }).signers([alice]).rpc();
    }catch(error){
    }
  });
  it("11) Alice cannot close Bob vault",async()=>{
    try{
      const tx = await program.methods.close().accounts({
        user : alice.publicKey,
        vault : vaultBobPDA,
        systemProgram : anchor.web3.SystemProgram.programId
      }).signers([alice]).rpc()
    }catch(error){
      assert.isTrue(error.toString().includes("Error") , "Should fail with signature mismatch error");
    }
  });
});

async function airdrop(connection: any, address: any, amount = 1 * anchor.web3.LAMPORTS_PER_SOL) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}