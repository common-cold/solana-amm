import { createInitializeMintInstruction, MINT_SIZE, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { FailedTransactionMetadata, LiteSVM } from "litesvm";


export function fundWallet(svm: LiteSVM, pubKey: PublicKey) {
    const res = svm.airdrop(pubKey, BigInt(10 * LAMPORTS_PER_SOL));
    console.log(res.toString());
}


export function initMint(
  svm: LiteSVM,
  coinMint: Keypair,
  payer: Keypair,
  owner: PublicKey,
  mintDecimals: number
) {
    console.log("Came hereeeee");
    const coinMintTx = new Transaction().add(
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: coinMint.publicKey,
            lamports: Number(svm.minimumBalanceForRentExemption(BigInt(MINT_SIZE))),
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
    coinMintTx.recentBlockhash = svm.latestBlockhash();
    coinMintTx.sign(payer, coinMint);
    const res = svm.sendTransaction(coinMintTx);

    console.log(res.toString());
}