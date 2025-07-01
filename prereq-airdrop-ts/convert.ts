import bs58 from "bs58";

const base58PublicKey = "2wt9vDBkLAwBmeWq4rSPSQ9ie2TLAjwvJByCVB8iuNa7";

// Decode the Base58 string into a Buffer
const publicKeyBuffer = bs58.decode(base58PublicKey);

// Output the buffer
console.log("📦 Buffer:", publicKeyBuffer);
console.log("🔢 Hex:", publicKeyBuffer.toString());
console.log("🧱 Byte Array:", Array.from(publicKeyBuffer));
