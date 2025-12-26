import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { LiteSVM } from "litesvm";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import fs from "node:fs";
import os from "os";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { ObjectId } from "bson";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { AmmAccount } from "./types";
import { expect } from "chai";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";

dotenv.config({ path: "./tests/.env" });

describe("amm", () => {
  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection("https://api.devnet.solana.com"),
    new anchor.Wallet(
      anchor.web3.Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(fs.readFileSync(`${os.homedir()}/.config/solana/id.json`, "utf8")))
      )
    ),
      {}
  );

  anchor.setProvider(provider);
  const program = anchor.workspace.amm as Program<Amm>;
  const ammOwner = Keypair.fromSecretKey(bs58.decode(process.env.KEYPAIR_1));
  

  let svm = new LiteSVM();
  let programId = new PublicKey("5qEiXgcAj5HRZLtmQgHEwPUCzKb9XqWqjztdSxHbxkV4");
  svm.addProgramFromFile(programId, "./target/deploy/amm.so");
  svm.airdrop(ammOwner.publicKey, BigInt(5 * LAMPORTS_PER_SOL));

  let AMM_ID = new ObjectId("694d55ce2acc8fab670e77d0");
  let SOL_DEVNET = new PublicKey("So11111111111111111111111111111111111111112");
  let USDC_DEVENT = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

  it("Initialize AMM", async () => {
    const [ammAccount, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("amm"), AMM_ID.id], programId 
    )

    const tx = await program.methods
    .initializeAmm(Array.from(AMM_ID.id), 30)
    .accounts({
      signer: ammOwner.publicKey,
      ammAccount: ammAccount,
      systemProgram: SYSTEM_PROGRAM_ID
    })
    .signers([ammOwner])
    .transaction();

    const recentBlockhash = svm.latestBlockhash();
    tx.recentBlockhash = recentBlockhash;
    tx.sign(ammOwner);

    const res = svm.sendTransaction(tx);

    const accountInfo = svm.getAccount(ammAccount);
    if (!accountInfo) {
      console.log("Empty account");
    }

    const data: AmmAccount = program.coder.accounts.decode("ammAccount", Buffer.from(accountInfo.data));
    
    expect(data.id).to.deep.equal(Array.from(AMM_ID.id));
    expect(data.owner.toBase58()).to.equal(ammOwner.publicKey.toBase58());
    expect(data.fee).to.equal(30);
  });

  it("Initialize Pool", async () => {
    try {
      const ammAccount = PublicKey.findProgramAddressSync(
        [Buffer.from("amm"), AMM_ID.id], programId 
      )[0];

      const poolAccount = PublicKey.findProgramAddressSync(
        [Buffer.from("pool"), AMM_ID.id, SOL_DEVNET.toBuffer(), USDC_DEVENT.toBuffer()], programId 
      )[0];

      const LPTokenMintAccount = PublicKey.findProgramAddressSync(
        [Buffer.from("lp_token"), AMM_ID.id, SOL_DEVNET.toBuffer(), USDC_DEVENT.toBuffer()], programId 
      )[0];

      const vaultA = getAssociatedTokenAddressSync(
        SOL_DEVNET,
        ammAccount,
        true,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

      const vaultB = getAssociatedTokenAddressSync(
        USDC_DEVENT,
        ammAccount,
        true,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

      console.log(ammAccount);
      const tx = await program.methods
      .initializePool(Array.from(AMM_ID.id), "LPToken", "LPT")
      .accounts({
        signer: ammOwner.publicKey,
        ammAccount: ammAccount,
        mintA: SOL_DEVNET,
        mintB: USDC_DEVENT,
        poolAccount: poolAccount,
        mintLiquidityToken: LPTokenMintAccount,
        vaultA: vaultA,
        vaultB: vaultB,

        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenProgram2022: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([ammOwner])
      .transaction();

      const recentBlockhash = svm.latestBlockhash();
      tx.recentBlockhash = recentBlockhash;
      tx.sign(ammOwner);

      const res = svm.sendTransaction(tx);

      console.log(res);

      // const accountInfo = svm.getAccount(ammAccount);
      // if (!accountInfo) {
      //   console.log("Empty account");
      // }

      // const data: AmmAccount = program.coder.accounts.decode("ammAccount", Buffer.from(accountInfo.data));
      
      // expect(data.id).to.deep.equal(Array.from(AMM_ID.id));
      // expect(data.owner.toBase58()).to.equal(ammOwner.publicKey.toBase58());
      // expect(data.fee).to.equal(30);
    } catch (e) {
      console.log(e);
    }
  });
    
});
