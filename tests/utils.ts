import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createInitializeMetadataPointerInstruction, createInitializeMintInstruction, ExtensionType, getAssociatedTokenAddressSync, getMintLen, getOrCreateAssociatedTokenAccount, LENGTH_SIZE, MINT_SIZE, mintTo, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID, TYPE_SIZE } from "@solana/spl-token";
import { createInitializeInstruction, pack } from "@solana/spl-token-metadata";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { FailedTransactionMetadata, LiteSVM } from "litesvm";


// export function fundWallet(svm: LiteSVM, pubKey: PublicKey) {
//     const res = svm.airdrop(pubKey, BigInt(10 * LAMPORTS_PER_SOL));
//     console.log(res.toString());
// }


// export function initMint(
//   svm: LiteSVM,
//   coinMint: Keypair,
//   payer: Keypair,
//   owner: PublicKey,
//   mintDecimals: number
// ) {
//     console.log("Came hereeeee");
//     const coinMintTx = new Transaction().add(
//         SystemProgram.createAccount({
//             fromPubkey: payer.publicKey,
//             newAccountPubkey: coinMint.publicKey,
//             lamports: Number(svm.minimumBalanceForRentExemption(BigInt(MINT_SIZE))),
//             space: MINT_SIZE,
//             programId: owner
//         }),

//         createInitializeMintInstruction(
//             coinMint.publicKey,
//             mintDecimals,
//             payer.publicKey,
//             null,
//             owner
//         )
//     );
//     coinMintTx.feePayer = payer.publicKey;
//     coinMintTx.recentBlockhash = svm.latestBlockhash();
//     coinMintTx.sign(payer, coinMint);
//     const res = svm.sendTransaction(coinMintTx);

//     console.log(res.toString());
// }

// export function makeTokenwithMetaData(svm: LiteSVM, mintAddress: PublicKey, poolAccount: PublicKey, payer: Keypair, owner: Keypair) {
//     const metadata = {
//         mint: mintAddress,
//         name: 'LiquidityToken',
//         symbol: 'LIQTOKEN',
//         uri: "",
//         additionalMetadata: [],
//     }
    
//     //used for serialization and deserialization
//     const metadataExtensionSize = TYPE_SIZE + LENGTH_SIZE;
//     const metadataSize = pack(metadata).length;
//     const mintLen = getMintLen([ExtensionType.MetadataPointer]);
//     const lamports = svm.getRent().minimumBalance(BigInt(mintLen + metadataSize + metadataExtensionSize));

//     const tx = new Transaction().add(
        
//         //create an account
//         SystemProgram.createAccount({
//             fromPubkey: payer.publicKey,
//             newAccountPubkey: mintAddress,
//             lamports: Number(lamports),
//             space: mintLen,
//             programId: TOKEN_2022_PROGRAM_ID
//         }),
        
//         //initilaize metadata pointer extension onto this mint acccount
//         createInitializeMetadataPointerInstruction(
//             mintAddress,
//             poolAccount,
//             mintAddress,
//             TOKEN_2022_PROGRAM_ID
//         ),

        
//         //initialize rest of the mint account data
//         createInitializeMintInstruction(
//             mintAddress,
//             9,
//             poolAccount,
//             poolAccount,
//             TOKEN_2022_PROGRAM_ID
//         ),

        
//         //initialize token metadata extension with required metadata (name, symbol, uri)
//         createInitializeInstruction({
//             programId: TOKEN_2022_PROGRAM_ID,
//             metadata: mintAddress,
//             updateAuthority: poolAccount,
//             mint: mintAddress,
//             mintAuthority: poolAccount,
//             name: metadata.name,
//             symbol: metadata.symbol,
//             uri: metadata.uri
//         })
//     );
//     console.log("OWNER: " + owner);
//     tx.feePayer = payer.publicKey;
//     tx.recentBlockhash = svm.latestBlockhash();
//     tx.sign(payer, owner);
//     const res = svm.sendTransaction(tx);

//     console.log(res.toString());
    
// }



export async function fundWallet(connection: Connection, pubKey: PublicKey) {
    const res = await connection.requestAirdrop(pubKey, 10 * LAMPORTS_PER_SOL);
    console.log(res.toString());
}


export async function initMint(
  connection: Connection,
  coinMint: Keypair,
  payer: Keypair,
  owner: PublicKey,
  mintDecimals: number
) {
    try {
        const lamports = Number(await connection.getMinimumBalanceForRentExemption(MINT_SIZE));
        const coinMintTx = new Transaction().add(
            SystemProgram.createAccount({
                fromPubkey: payer.publicKey,
                newAccountPubkey: coinMint.publicKey,
                lamports: lamports,
                space: MINT_SIZE,
                programId: owner
            }),

            createInitializeMintInstruction(
                coinMint.publicKey,
                mintDecimals,
                payer.publicKey,
                null,
                owner
            )
        );
        coinMintTx.feePayer = payer.publicKey;
        const latestBlockhash = await connection.getLatestBlockhash();
        coinMintTx.recentBlockhash = latestBlockhash.blockhash;
        const res = await connection.sendTransaction(coinMintTx, [payer, coinMint]);

        console.log(res.toString());
    } catch (e) {
        console.log(e);
    }
}

export async function getOrCreateAta(connection: Connection, mintOwner: Keypair, mint: PublicKey, user: PublicKey) {
    const ata = getAssociatedTokenAddressSync(
        mint,
        user,
        false,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const accountInfo = await connection.getAccountInfo(ata);
    if (!accountInfo) {
        console.log("Creating ATA");
        let ataAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            mintOwner,
            mint,
            user,
            false,
            null,
            null,
            TOKEN_2022_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );
        console.log("RES: " + JSON.stringify(ataAccount));
        return ataAccount.address;
    } else {
        console.log("ATA already created");
        return ata;
    }
}

export async function mintTokenToUser(connection: Connection, mintOwner: Keypair, mint: PublicKey, to: PublicKey, amount: bigint) {
    const tx = await mintTo(connection, mintOwner, mint, to, mintOwner, amount);
    console.log(tx);
}