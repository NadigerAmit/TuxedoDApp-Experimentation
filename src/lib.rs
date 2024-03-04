use wasm_bindgen::prelude::*;

use sp_core::H256;
/*
use tuxedo_core::{
    dynamic_typing::UtxoData,
    types::{Input, Output, OutputRef},
};
*/

// If above code is uncommented we will below error :
/*
error[E0152]: duplicate lang item in crate `sp_io` (which `sp_application_crypto` depends on): `panic_impl`.
  |
  = note: the lang item is first defined in crate `std` (which `TuxedoDapp` depends on)
  = note: first definition in `std` loaded from /home/amit/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-unknown/lib/libstd-ac0e2369215361d6.rlib
  = note: second definition in `sp_io` loaded from /home/amit/TuxedoDApp/TuxedoDapp/target/wasm32-unknown-unknown/release/deps/libsp_io-ad9f48a6dbb1a75e.rlib, /home/amit/TuxedoDApp/TuxedoDapp/target/wasm32-unknown-unknown/release/deps/libsp_io-ad9f48a6dbb1a75e.rmeta

For more information about this error, try `rustc --explain E0152`
*/



#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello Greeting from Tuxedo DApp, {}!", name));
}

#[wasm_bindgen]
pub fn get_kitty_info(id: u32) -> String {
    // Implement your logic to retrieve information about a kitty with ID 'id' from the UTXO-based Substrate blockchain
    // This could involve interacting with the Tuxedo API or custom blockchain calls
    // For this example, let's just return a placeholder string
    format!("Gett Kitty info called from Rust code ")
}