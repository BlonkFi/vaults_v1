import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

export async function generateFundedKeypair(connection: Connection) {
  const keypair = Keypair.generate();

  const tx = await connection.requestAirdrop(
    keypair.publicKey,
    1 * LAMPORTS_PER_SOL
  );

  const latestBlockhash = await connection.getLatestBlockhash();

  await connection.confirmTransaction({
    signature: tx,
    blockhash: latestBlockhash.blockhash,
    lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
  });

  return keypair;
}

export function createLocalhostConnection() {
  return new Connection("http://127.0.0.1:8899", "confirmed");
}
