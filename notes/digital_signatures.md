# Digital Signatures

Create a private key using openssl or the libopenssl bindings of your favourite language:

```bash
# only one of them is necessary
openssl genpkey -algorithm ED25519 -out private.pem # signature length  64
openssl genpkey -algorithm ED448   -out private.pem # signature length 114
openssl genpkey -algorithm RSA     -out private.pem # signature length 256 (for 2048-bit keys)
```

Derive the corresponding public key:

```bash
openssl pkey -in private.pem -pubout -out public.pem
```

Derive the corresponding keys in DER format:

```
openssl pkey -in private.pem         -outform DER -out private.der
openssl pkey -in private.pem -pubout -outform DER -out public.der
```

## Bash

- ED25519 and ED448 require `-rawin` from OpenSSL 3.0 alpha 1 (https://github.com/openssl/openssl/issues/6988)

```sh
echo -n '{"TankMove":{"direction":"Up"}}' > data
openssl pkeyutl -inkey private.pem        -in data -rawin -sign          > signature
openssl pkeyutl -inkey public.pem  -pubin -in data -rawin -verify -sigfile signature

echo "message: `cat data`"
echo "signature: `od -t u1 signature | cut -f 2- -d " " -s`"
```

## Rust

```rs
use board::{
	Action::{TankMove},
	Direction::{Up},
};
use openssl::{
	pkey::PKey,
	sign::{Signer, Verifier},
};

fn main() {
	let data = serde_json::to_string(&TankMove { direction: Up }).unwrap();

	let private_key =
		PKey::private_key_from_pem(&std::fs::read("private.pem").unwrap()).unwrap();
	let mut signer = Signer::new_without_digest(&private_key).unwrap();
	let signature = signer.sign_oneshot_to_vec(data.as_bytes()).unwrap();

	let public_key = PKey::public_key_from_pem(&std::fs::read("public.pem").unwrap()).unwrap();
	let mut verifier = Verifier::new_without_digest(&public_key).unwrap();
	let valid = verifier
		.verify_oneshot(&signature, data.as_bytes())
		.unwrap();

	println!("message: {:}", data);
	println!("signature: {:?}", signature);
	assert!(valid);
}
```

## Javascript (Node.JS)

```js
import { promisify } from "util"
import { createPrivateKey, createPublicKey, sign, verify } from "crypto"
import { readFile } from "fs/promises"

const data = Buffer.from(
	JSON.stringify({ TankMove: { direction: "Up" } }),
	"utf-8",
)

const privateKey = createPrivateKey(await readFile("private.pem"))
const signature = await promisify(sign)(null, data, privateKey)

const publicKey = createPublicKey(await readFile("public.pem"))
const valid = await promisify(verify)(null, data, publicKey, signature)

console.log("message:", data.toString("utf-8"))
console.log("signature:", Array.from(signature))
console.assert(valid)
```

## Javascript (Web)

```js
// const readFile = ...

const data = new TextEncoder("utf-8")
	.encode(JSON.stringify({ TankMove: { direction: "Up" } }))

const privateKey = await crypto.subtle.importKey("pkcs8", await readFile("private.der"), "Ed25519", false, ["sign"])
const signature = await crypto.subtle.sign("Ed25519", privateKey, data)

const publicKey = await crypto.subtle.importKey("spki", await readFile("public.der"), "Ed25519", false, ["verify"])
const valid = await crypto.subtle.verify("Ed25519", publicKey, signature, data)

console.log("message:", new TextDecoder("utf-8").decode(data))
console.log("signature:", new Uint8Array(signature))
console.assert(valid)
```
