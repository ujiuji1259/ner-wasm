use sudachi::tokenizer::{Tokenizer, Mode};

// #[wasm_bindgen]
pub struct MyTokenizer {
    tokenizer: Tokenizer<'static>,
}

// #[wasm_bindgen]
impl MyTokenizer {
    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        let bytes = include_bytes!("/Users/ujiie/sudachi.rs/src/resources/system.dic");
        Self {
            tokenizer: Tokenizer::new(bytes)
        }
    }

    // pub fn tokenize(&self, s: &str) -> Array {
    pub fn tokenize(&self, s: &str) -> Vec<String> {
        let morpheme_list = self.tokenizer.tokenize(&s.to_string(), &Mode::B, false);

        // let tokens: Array = morpheme_list
        let tokens: Vec<String> = morpheme_list
            .iter()
            // .map(|m| JsString::from(m.surface().clone()))
            .map(|m| String::from(m.surface()))
            .collect();
        return tokens;
    }
}