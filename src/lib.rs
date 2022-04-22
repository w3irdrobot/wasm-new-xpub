// use std::fmt;
// use wasm_bindgen::convert::IntoWasmAbi;
use bip32::{ChildNumber, Prefix, XPrv};
use bip39::{Language, Mnemonic};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct XPub {
    mnemonic: String,
    xpub: String,
    fingerprint: String,
}

// #[wasm_bindgen]
// pub enum Error {
//     MnemonicGenerate,
//     PrivateKeyCreate,
//     ChildDerive,
// }

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let repr = match self {
//             Error::MnemonicGenerate => "MnemonicGenerate",
//             Error::PrivateKeyCreate => "PrivateKeyCreate",
//             Error::ChildDerive => "ChildDerive",
//             _ => "Generic",
//         };

//         write!(f, "{}", repr)
//     }
// }

// impl IntoWasmAbi for Error {
//     type Abi = <String as IntoWasmAbi>::Abi;

//     fn into_abi(self) -> Self::Abi {
//         self.to_string().into_abi()
//     }
// }

#[wasm_bindgen]
impl XPub {
    pub fn new() -> XPub {
        let mut rng = rand::thread_rng();
        let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 24).unwrap();
        let master = XPrv::new(mnemonic.to_seed("password")).unwrap();
        let key_path = ChildNumber::new(84, true).unwrap();
        let account = master.derive_child(key_path).unwrap();
        let pub_key = account.public_key();
        let fingerprint = hex::encode(pub_key.fingerprint());

        XPub {
            mnemonic: mnemonic.to_string(),
            fingerprint,
            xpub: pub_key.to_string(Prefix::XPUB),
        }
    }

    pub fn mnemonic(&self) -> String {
        self.mnemonic.clone()
    }

    pub fn xpub(&self) -> String {
        self.xpub.clone()
    }

    pub fn fingerprint(&self) -> String {
        self.fingerprint.clone()
    }
}
