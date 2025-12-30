import { ASSOCIATED_TOKEN_PROGRAM_ID, createInitializeMetadataPointerInstruction, createInitializeMintInstruction, ExtensionType, getAssociatedTokenAddressSync, getMintLen, getOrCreateAssociatedTokenAccount, LENGTH_SIZE, MINT_SIZE, mintTo, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID, TYPE_SIZE } from "@solana/spl-token";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";


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