import bs58 from "bs58";

function walletToBase58() {
  const wallet: number[] = [
    119, 142, 156, 187, 162, 208, 117, 206, 232, 246, 124, 1, 34, 191, 13, 210, 195, 163, 35, 10, 40, 141, 104, 101,
    248, 30, 208, 191, 249, 147, 190, 17,
  ];
  const base58 = bs58.encode(Buffer.from(wallet));
  console.log("Base58 Encoded String:", base58);
}

walletToBase58();
