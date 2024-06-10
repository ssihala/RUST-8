
mod chip8;
mod interface;


use chip8::Chip8;
use crate::interface::Interface;


fn main() -> Result<(), String>{
    let mut args = std::env::args();    
    let rom_name = args.nth(1).unwrap();
    let rom_folder_path = String::from("./roms/");
    let rom_path = rom_folder_path+&rom_name;
    let cycle_speed_arg = args.next().unwrap();
    let cycle_speed :i32 = cycle_speed_arg.parse().expect("Can't parse cycle speed, enter an integer");

    let mut chip = Chip8::new(cycle_speed);
    chip.load_rom(&rom_path);
    chip.load_font();

    let interface = Interface::new(String::from("RUST-8"), 64, 32, 25);
       
    let _ = interface.window_loop(&mut chip);

    Ok(())
}
