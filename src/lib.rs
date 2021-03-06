extern crate console_error_panic_hook;

use sudachi::tokenizer::{Tokenizer, Mode};
use js_sys::{Array, JsString, global, Object};
use wasm_bindgen::prelude::*;
use std::path::Path;
use crfs::{Model, Attribute, Tagger};


// #[wasm_bindgen]
fn extract_features(sent: &Vec<String>) -> Vec<Vec<Attribute>> {
    let mut result: Vec<Vec<Attribute>> = Vec::new();
    for idx in 0..sent.len() {
        let mut attributes: Vec<Attribute> = Vec::new();
        attributes.push(Attribute::new(&sent[idx], 1.0));

        let pre_word = match idx {
            0 => "BOS",
            _ => &sent[idx-1],
        };

        let post_word = match idx {
            n if n >= sent.len() - 1 => "EOS",
            _ => &sent[idx+1],
        };

        attributes.push(Attribute::new(format!("-1_{}", pre_word), 1.0));
        attributes.push(Attribute::new(format!("{}_{}", pre_word, &sent[idx]), 1.0));

        attributes.push(Attribute::new(format!("+1_{}", post_word), 1.0));
        attributes.push(Attribute::new(format!("{}_{}", &sent[idx], post_word), 1.0));

        result.push(attributes);
    }
    return result;
}


#[wasm_bindgen]
pub struct NER {
    tagger: Model<'static>,
    tokenizer: Tokenizer<'static>,
}

impl NER {
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


#[wasm_bindgen]
impl NER {
    pub fn new() -> Self {
        // data for crf
        let buf = include_bytes!("/path/to/ner-wasm/data_im.crfsuite");

        // data for tokenizer
        let bytes = include_bytes!("/path/to/sudachi.rs/src/resources/system.dic");
        Self {
            tagger: Model::new(buf).unwrap(),
            tokenizer: Tokenizer::new(bytes),
        }
    }

    pub fn tag(&mut self, sent: &str) -> Array {
        let xseq = self.tokenize(sent);

        let mut tagger = self.tagger.tagger().unwrap();
        let attributes = extract_features(&xseq);
        let res = tagger.tag(&attributes).unwrap();
        let yseq: Array = res.iter()
            .map(|s| JsString::from(s.to_string()))
            // .map(|(x, y)| Array::from(JsString::from(x.clone()), JsString::from(y.to_string())))
            .collect();
        let xseq: Array = xseq
            .iter()
            // .map(|m| JsString::from(m.surface().clone()))
            .map(|s| JsString::from(s.clone()))
            .collect();
        // (xseq, yseq)
        // xseq.concat(&yseq)
        Array::of2(&xseq, &yseq)
        // res
    }
}


#[cfg(test)]
mod tests {
    use crate::{MyTokenizer, NER};
    #[test]
    fn it_works() {
        // let tokenizer = MyTokenizer::new();
        let text = "????????????????????????????????????";
        // let tokens = tokenizer.tokenize(text);
        // println!("{:?}", tokens);
        let mut model = NER::new();
        // let tokens = vec![
        //     String::from("???"),
        //     String::from("???"),
        //     String::from("??????"),
        //     String::from("???")
        // ];
        let res = model.tag(text);
        assert_eq!(1+1, 2);
    }
}
