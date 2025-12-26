import { PublicKey } from "@solana/web3.js"

export type AmmAccount = {
    id: Uint8Array,
    owner: PublicKey,
    fee: number
}