import bs58 from "bs58";
import * as readline from "readline";

// Function to decode a Base58 string into a wallet format
function base58ToWallet() {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  rl.question("Enter your Base58 string: ", (base58) => {
    try {
      const wallet = bs58.decode(base58);
      console.log("Decoded Wallet:", wallet);
    } catch (error) {
      console.error("Invalid Base58 input:", error);
    }
    rl.close();
  });
}

base58ToWallet();
