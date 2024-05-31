
mod chip8;
mod interface;

use std::sync::Arc;

use chip8::Chip8;
use crate::interface::Interface;


fn main() -> Result<(), String>{

    let mut chip = Chip8::new();
    chip.load_rom(&String::from("3-corax+.ch8"));
    // chip.load_rom(&String::from("IBM Logo.ch8"));
    // chip.load_rom(&String::from("1-chip8-logo.ch8"));
    chip.load_font();


    let interface = Interface::new(String::from("RUST-8"), 64, 32, 16);
    let mut canvas = interface.window_loop(&mut chip).expect("Error initializing window");
    
    
    interface.window_loop(&mut chip);

    Ok(())
}
