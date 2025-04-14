import wallet from "/home/sabb/.config/solana/id.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader({ address: "https://devnet.irys.xyz/" }));
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        //2. Convert image to generic file.
        //3. Upload image

        const image = await readFile("./sabb.jpg");

        const file = createGenericFile(image, "sabb.jpg", { contentType: "image/jpg" });

        const [myUri] = await umi.uploader.upload([ file ])
        console.log("Your image URI: ", myUri);

        const irysURI = myUri.replace(
            "https://arweave.net/",
            "https://devnet.irys.xyz/"
        );

        console.log(irysURI)
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();

// https://turbine-solanad-4cde.devnet.rpcpool.com/168dd64f-ce5e-4e19-a836-f6482ad6b396
// umi.use(irysUploader({address: "https://devnet.irys.xyz/",}));
