import {
  Signature,
  PublicKey,
  PrivateKey,
  Field
} from 'o1js';

export async function verify(json) {

  const parsedJson = JSON.parse(json);
  const pubKey = PublicKey.fromBase58(parsedJson.public_key);

  // const dataFields: Field[] = [];
  // parsedJson.data[0].forEach((byte: number) => dataFields.push(Field(byte)));

  const dataFields = [Field(42), Field(69), Field(28)];
  // Rust generated signature validation
  const rustSignature = Signature.fromBase58(parsedJson.signature);
  const isValidRust = rustSignature.verify(pubKey, dataFields);
  console.log('Rust signature is valid:', isValidRust.toBoolean());

  // Locally generated signature validation
  const privKey = PrivateKey.fromBase58(
    'EKFSmntAEAPm5CnYMsVpfSEuyNfbXfxy2vHW8HPxGyPPgm5xyRtN'
  );
  pubKey.assertEquals(privKey.toPublicKey());

  // const o1jsSignature = Signature.create(privKey, dataFields);
  // const isValidO1js = o1jsSignature.verify(pubKey, dataFields);
  // console.log('o1jsSignature', o1jsSignature.toBase58());
  // console.log('o1js signature r', o1jsSignature.r.toBigInt());
  // console.log('o1js signature s', o1jsSignature.s.toBigInt());
  // console.log('o1js signature is valid:', isValidO1js.toBoolean());

}
