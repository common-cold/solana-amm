import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { Keypair, PublicKey } from "@solana/web3.js";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { ObjectId } from "bson";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import dotenv from "dotenv";

dotenv.config({ path: "./tests/.env" });

describe("amm", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.amm as Program<Amm>;

    const mintOwner = Keypair.fromSecretKey(bs58.decode(process.env.MINT_OWNER));
    const ammOwner = Keypair.fromSecretKey(bs58.decode(process.env.KEYPAIR_1));

    let AMM_ID = new ObjectId("694d55ce2acc8fab670e77d0");
    
    let SOL_DEVNET = new PublicKey("So11111111111111111111111111111111111111112");

    let USDC_DEVENT = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")

    it("Initialize AMM", async () => {
        const [ammAccount, bump] = PublicKey.findProgramAddressSync(
        [Buffer.from("amm"), AMM_ID.id], program.programId 
        )

        const tx = await program.methods
        .initializeAmm(Array.from(AMM_ID.id), 30)
        .accounts({
        signer: ammOwner.publicKey,
        ammAccount: ammAccount,
        systemProgram: SYSTEM_PROGRAM_ID
        })
        .signers([ammOwner])
        .rpc();

        console.log(tx);

        // const accountInfo = svm.getAccount(ammAccount);
        // if (!accountInfo) {
        // console.log("Empty account");
        // }

        // const data: AmmAccount = program.coder.accounts.decode("ammAccount", Buffer.from(accountInfo.data));
        
        // expect(data.id).to.deep.equal(Array.from(AMM_ID.id));
        // expect(data.owner.toBase58()).to.equal(ammOwner.publicKey.toBase58());
        // expect(data.fee).to.equal(30);
    });

    it("Initialize Pool", async () => {
        try {
        const ammAccount = PublicKey.findProgramAddressSync(
            [Buffer.from("amm"), AMM_ID.id], program.programId
        )[0];

        const poolAccount = PublicKey.findProgramAddressSync(
            [Buffer.from("pool"), AMM_ID.id, SOL_DEVNET.toBuffer(), USDC_DEVENT.toBuffer()], program.programId 
        )[0];

        // const LPTokenMintAccount = PublicKey.findProgramAddressSync(
        //   [Buffer.from("lp_token"), AMM_ID.id, SOL_DEVNET.publicKey.toBuffer(), USDC_DEVENT.publicKey.toBuffer()], programId 
        // )[0];
        const LPTokenMintAccount = new Keypair();

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

        console.log("AMM OWNER: " + ammOwner.publicKey)
        console.log("AMM Account: " + ammAccount)
        console.log("SOL_DEVNET: " + SOL_DEVNET)
        console.log("USDC_DEVNET: " + USDC_DEVENT)
        console.log("Pool Account: " + poolAccount)
        console.log("LPTokenMintAccount: " + LPTokenMintAccount.publicKey)
        console.log("Vault A: " + vaultA)
        console.log("Vault B: " + vaultB)

        const tx = await program.methods
        .initializePool(Array.from(AMM_ID.id), "LPToken", "LPT")
        .accounts({
            signer: ammOwner.publicKey,
            ammAccount: ammAccount,
            mintA: SOL_DEVNET,
            mintB: USDC_DEVENT,
            poolAccount: poolAccount,
            mintLiquidityToken: LPTokenMintAccount.publicKey,
            vaultA: vaultA,
            vaultB: vaultB,

            systemProgram: SYSTEM_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID,
            tokenProgram2022: TOKEN_2022_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .signers([ammOwner, LPTokenMintAccount])
        .rpc();

        console.log(tx);
        // const recentBlockhash = svm.latestBlockhash();
        // tx.recentBlockhash = recentBlockhash;
        // tx.sign(ammOwner, LPTokenMintAccount);

        // const res = svm.sendTransaction(tx);

        // console.log(res.toString());

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