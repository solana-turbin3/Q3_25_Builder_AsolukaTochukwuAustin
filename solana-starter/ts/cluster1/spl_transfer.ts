import { Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("GG6JERkH8t8Wcmt6vjH3mCB8vz8fwv4r52vrLwSUGgws");

// Recipient address
const to = new PublicKey("3NPHMMM5dNde1ZV8VPkFQrd2TEPH6n9WTyPt3AyvfxdD");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const from = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);
        console.log(`Your from token account is: ${from.address.toBase58()}`)

        // Get the token account of the toWallet address, and if it does not exist, create it
        const receiver = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to);

        // Transfer the new token to the "toTokenAccount" we just created
        const tx = transfer(
            connection,
            keypair,
            from.address,
            receiver.address,
            keypair.publicKey,
            1_000_000,
        )
        console.log(`Success! Check out your TX here: ${tx}`)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();