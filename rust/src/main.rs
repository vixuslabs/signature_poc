use mina_signer::{
    self, BaseField, Keypair, NetworkId, PubKey, SecKey, Signature as MinaSignature, Signer,
};

use bitcoin;
use mina_hasher::{Hashable, ROInput};
use o1_utils::FieldHelpers;
use serde_json::json;
use std::fs::File;
use std::io::Write;

/// Data Struct
#[derive(Clone)]
pub struct Data {
    pub fields: Vec<BaseField>,
}

impl Hashable for Data {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        self.fields
            .iter()
            .fold(ROInput::new(), |roi, field| roi.append_field(*field))
    }

    fn domain_string(network_id: NetworkId) -> Option<String> {
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet".to_string().into(),
            NetworkId::TESTNET => "CodaSignature".to_string().into(),
        }
    }
}

pub enum Signature {
    Mina(MinaSignature),
}

impl Signature {
    /// Returns the bytes of this signature.
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Mina(sig) => {
                let mut bytes = Vec::with_capacity(BaseField::size_in_bytes() * 2);

                let rx_bytes = sig.rx.to_bytes();
                let s_bytes = sig.s.to_bytes();

                bytes.extend_from_slice(&rx_bytes);
                bytes.extend_from_slice(&s_bytes);
                bytes
            }
        }
    }
}

fn main() {
    let priv_key =
        SecKey::from_base58("EKFSmntAEAPm5CnYMsVpfSEuyNfbXfxy2vHW8HPxGyPPgm5xyRtN").unwrap();
    let keypair = Keypair::from_secret_key(priv_key.clone()).unwrap();

    let data = Data {
        fields: vec![BaseField::from(1u64), BaseField::from(2u64)],
    };
    let mut ctx = mina_signer::create_kimchi::<Data>(NetworkId::TESTNET);
    let signature = Signature::Mina(ctx.sign(&keypair, &data));
    let sig_bytes = [vec![154], vec![1], signature.to_bytes().to_vec()].concat();
    let b58_str = bitcoin::base58::encode_check(&sig_bytes);

    let json_data: serde_json::Value;
    match signature {
        Signature::Mina(sig) => {
            println!("rx: {:?}\ns: {:?}", sig.rx.to_biguint(), sig.s.to_biguint());
            json_data = json!({
                "data": data.fields.iter().map(|f| f.to_biguint()).collect::<Vec<_>>(),
                "signature": b58_str,
                "public_key": PubKey::from_secret_key(priv_key).unwrap().into_address(),
            });
        }
    }

    let json_string = serde_json::to_string_pretty(&json_data).unwrap();

    let mut file = File::create("../o1js/web/signature_poc.json").unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
}
