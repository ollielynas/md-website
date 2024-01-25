use wasm_bindgen::prelude::wasm_bindgen;
use web_sugars::{error::WebSysSugarsError, window::get_window};
use web_sys::{SpeechSynthesisUtterance, SpeechSynthesis};

use crate::err_to_sugar;





#[wasm_bindgen]
pub async fn read_message(text: &str) -> Result<(), WebSysSugarsError> {
    return Ok(());
    let speech = err_to_sugar(SpeechSynthesisUtterance::new_with_text(text))?;
    
    let synth: SpeechSynthesis = err_to_sugar(get_window()?.speech_synthesis())?;
    // synth.cancel();\
    if synth.speaking() {
        synth.cancel();
    }
    synth.speak(&speech);    
    return Ok(());
}