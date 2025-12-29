import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { ObjectId } from "bson";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAccount, getAssociatedTokenAddressSync, getMint, getOrCreateAssociatedTokenAccount, getTokenMetadata, mintTo, NATIVE_MINT, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import dotenv from "dotenv";
import { expect } from "chai";
import { getOrCreateAta, mintTokenToUser } from "./utils";
import { BN } from "bn.js";

dotenv.config({ path: "./tests/.env" });


describe("amm", async function () {
    this.timeout(300_000);

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.amm as Program<Amm>;

    const connection = new Connection("https://api.devnet.solana.com");

    const mintOwner = Keypair.fromSecretKey(bs58.decode(process.env.MINT_OWNER));
    const tokenAMint = Keypair.fromSecretKey(bs58.decode(process.env.TOKEN_A_MINT)); //9 decimals
    const tokenBMint = Keypair.fromSecretKey(bs58.decode(process.env.TOKEN_B_MINT)); //6 decimals
    const ammOwner = Keypair.fromSecretKey(bs58.decode(process.env.KEYPAIR_1));
    const user = Keypair.fromSecretKey(bs58.decode(process.env.KEYPAIR_2));

    let AMM_ID = new ObjectId("694d55ce2acc8fab670e77d2");
    let TOKEN_A_DEVNET = tokenAMint.publicKey;
    let TOKEN_A_DECIMALS = 9;
    let TOKEN_B_DEVNET = tokenBMint.publicKey;
    let TOKEN_B_DECIMALS = 6;

    const ammAccount = PublicKey.findProgramAddressSync(
        [Buffer.from("amm"), AMM_ID.id], program.programId
    )[0];

    const poolAccount = PublicKey.findProgramAddressSync(
        [Buffer.from("pool"), AMM_ID.id, TOKEN_A_DEVNET.toBuffer(), TOKEN_B_DEVNET.toBuffer()], program.programId 
    )[0];

    const LPTokenMintAccount = PublicKey.findProgramAddressSync(
        [Buffer.from("lp_token"), poolAccount.toBuffer()], program.programId 
    )[0];

    let LP_TOKEN_DECIMALS = 9;

    const lockPda = PublicKey.findProgramAddressSync(
        [Buffer.from("lock_pda"), poolAccount.toBuffer()], program.programId 
    )[0];

    const lockedLpAta = getAssociatedTokenAddressSync(
        LPTokenMintAccount,
        lockPda,
        true,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );


    const vaultA = getAssociatedTokenAddressSync(
        TOKEN_A_DEVNET,
        poolAccount,
        true,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );


    const vaultB = getAssociatedTokenAddressSync(
        TOKEN_B_DEVNET,
        poolAccount,
        true,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const userLiquidityTokenAta = getAssociatedTokenAddressSync(
        LPTokenMintAccount,
        user.publicKey,
        false,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );

    // let userAtaA = await getOrCreateAta(connection, mintOwner, TOKEN_A_DEVNET, user.publicKey);

    // let userAtaB = await getOrCreateAta(connection, mintOwner, TOKEN_B_DEVNET, user.publicKey);

    let userAtaA = new PublicKey("BnoEZys6keoSWMHYc99Ah6NHhGZbsFg2ZumeZNdUF47G");
    let userAtaB = new PublicKey("8QbhWQuMMpYWAdoRhNGxmXbYXuh3F87jx1ENSYLAuYj3");

    const TOKEN_NAME = "LPToken";
    const TOKEN_SYMBOL = "LPT";
    

    console.log("AMM OWNER: " + ammOwner.publicKey)
    console.log("AMM Account: " + ammAccount)
    console.log("TOKEN_A_DEVNET: " + TOKEN_A_DEVNET)
    console.log("TOKEN_B_DEVNET: " + TOKEN_B_DEVNET)
    console.log("Pool Account: " + poolAccount)
    console.log("LPTokenMintAccount: " + LPTokenMintAccount)
    console.log("Lock Pda: " + lockPda);
    console.log("Locked LP Ata: " + lockedLpAta);
    console.log("Vault A: " + vaultA)
    console.log("Vault B: " + vaultB)
    console.log("User LP TOken Ata: " + userLiquidityTokenAta);
    console.log("User Ata A: " + userAtaA)
    console.log("User Ata A: " + userAtaB)



    // it("Initialize AMM", async () => {
    //     const ammAccountInfo = await connection.getAccountInfo(ammAccount);
        
    //     if (!ammAccountInfo) {
    //         const tx = await program.methods
    //         .initializeAmm(Array.from(AMM_ID.id), 30)
    //         .accounts({
    //         signer: ammOwner.publicKey,
    //         ammAccount: ammAccount,
    //         systemProgram: SYSTEM_PROGRAM_ID
    //         })
    //         .signers([ammOwner])
    //         .rpc();

    //         console.log(tx);
    //     }

    //     const data = await program.account.ammAccount.fetch(ammAccount);
    //     expect(data.id).to.deep.equal(Array.from(AMM_ID.id));
    //     expect(data.owner.toBase58()).to.equal(ammOwner.publicKey.toBase58());
    //     expect(data.fee).to.equal(30);
    // });

    // it("Initialize Pool", async () => {
    //     const poolAccountInfo = await connection.getAccountInfo(poolAccount);
    //     if (!poolAccountInfo) {
    //         const tx = await program.methods
    //         .initializePool(Array.from(AMM_ID.id), TOKEN_NAME, TOKEN_SYMBOL)
    //         .accounts({
    //             signer: ammOwner.publicKey,
    //             ammAccount: ammAccount,
    //             mintA: TOKEN_A_DEVNET,
    //             mintB: TOKEN_B_DEVNET,
    //             poolAccount: poolAccount,
    //             mintLiquidityToken: LPTokenMintAccount,
    //             vaultA: vaultA,
    //             vaultB: vaultB,

    //             systemProgram: SYSTEM_PROGRAM_ID,
    //             tokenProgram: TOKEN_PROGRAM_ID,
    //             tokenProgram2022: TOKEN_2022_PROGRAM_ID,
    //             associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    //         })
    //         .signers([ammOwner])
    //         .rpc();

    //         console.log(tx);
    //     }

    //     const poolData = await program.account.poolAccount.fetch(poolAccount);
    //     expect(poolData.poolAuthority.toBase58()).to.equal(ammAccount.toBase58());
    //     expect(poolData.mintA.toBase58()).to.equal(TOKEN_A_DEVNET.toBase58());
    //     expect(poolData.mintB.toBase58()).to.equal(TOKEN_B_DEVNET.toBase58());
    //     expect(poolData.mintLiquidityToken.toBase58()).to.equal(LPTokenMintAccount.toBase58());
    //     expect(poolData.vaultA.toBase58()).to.equal(vaultA.toBase58());
    //     expect(poolData.vaultB.toBase58()).to.equal(vaultB.toBase58());

    //     const LPTokenMintData = await getMint(connection, LPTokenMintAccount, null, TOKEN_2022_PROGRAM_ID);
    //     expect(LPTokenMintData.decimals).to.equal(9);
    //     expect(LPTokenMintData.freezeAuthority.toBase58()).to.equal(poolAccount.toBase58());
    //     expect(LPTokenMintData.mintAuthority.toBase58()).to.equal(poolAccount.toBase58()); 
    //     expect(LPTokenMintData.supply).to.equal(BigInt(0));

    //     const LPTokenMetadata = await getTokenMetadata(connection, LPTokenMintAccount);
    //     expect(LPTokenMetadata.name).to.equal(TOKEN_NAME);
    //     expect(LPTokenMetadata.symbol).to.equal(TOKEN_SYMBOL);
    //     expect(LPTokenMetadata.updateAuthority.toBase58()).to.equal(poolAccount.toBase58());

    //     await mintTokenToUser(connection, mintOwner, TOKEN_A_DEVNET, userAtaA,  BigInt(5 * Math.pow(10, TOKEN_A_DECIMALS)));
    //     await mintTokenToUser(connection, mintOwner, TOKEN_B_DEVNET, userAtaB,  BigInt(500 * Math.pow(10, TOKEN_B_DECIMALS)));
    // });



    // it("Deposit Liquidity For First Time", async () => {
        //Let Market Price:
        // 1 TOKEN A = 150 TOKEN B
       
        // const tx = await program.methods
        // .depositLiquidity(Array.from(AMM_ID.id), new BN(1 * Math.pow(10, TOKEN_A_DECIMALS)), new BN(150 * Math.pow(10, TOKEN_B_DECIMALS)))
        // .accounts({
        //     signer: user.publicKey,
        //     mintA: TOKEN_A_DEVNET,
        //     mintB: TOKEN_B_DEVNET,
        //     poolAccount: poolAccount,
        //     mintLiquidityToken: LPTokenMintAccount,
        //     vaultA: vaultA,
        //     vaultB: vaultB,
        //     userAtaA: userAtaA,
        //     userAtaB: userAtaB,
        //     userLiquidityTokenAta: userLiquidityTokenAta,
        //     lockPda: lockPda,
        //     lockedLpAta: lockedLpAta,
        //     systemProgram: SYSTEM_PROGRAM_ID,
        //     tokenProgram: TOKEN_PROGRAM_ID,
        //     tokenProgram2022: TOKEN_2022_PROGRAM_ID,
        //     associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        // })
        // .signers([user])
        // .rpc();

        // console.log(tx);

    // });
});