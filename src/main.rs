
mod chip8;
mod interface;

use chip8::Chip8;
use crate::interface::Interface;


fn main() -> Result<(), String>{

    let mut chip = Chip8::new(500);
    chip.load_rom(&String::from("pong.rom"));
    // chip.load_rom(&String::from("2-ibm-logo.ch8"));
    // chip.load_rom(&String::from("1-chip8-logo.ch8"));
    chip.load_font();


    let interface = Interface::new(String::from("RUST-8"), 64, 32, 25);
    
    
    
    let _ = interface.window_loop(&mut chip);

    Ok(())
}
