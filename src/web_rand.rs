use rand::RngCore;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Crypto;

#[wasm_bindgen]
pub struct WebRand(Crypto);

impl WebRand {
    pub fn get() -> Option<Self> {
        web_sys::window()
            .and_then(|window| window.crypto().ok())
            .map(WebRand)
    }

    pub fn generate_seed(&self) -> [u8; 32] {
        let mut dest = [0u8; 32];
        self.0.get_random_values_with_u8_array(&mut dest).unwrap();
        dest
    }
}

impl RngCore for WebRand {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        self.fill_bytes(&mut bytes);
        u32::from_le_bytes(bytes)
    }

    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0u8; 8];
        self.fill_bytes(&mut bytes);
        u64::from_le_bytes(bytes)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.get_random_values_with_u8_array(dest).unwrap();
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        Ok(self.fill_bytes(dest))
    }
}
