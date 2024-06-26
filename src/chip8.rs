use std::fs::File;
use std::io::Read;
use rand::random;

pub enum Key{
    Pressed,
    Released
}

pub struct Chip8 {
    memory: [u8; 4096],
    font: [u8; 80],
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    //64 columns, 32 rows
    display: [bool; 64*32],
    keypad: [bool; 16],
    pc: u16,
    index_register: u16,
    registers: [u8; 16],
    cycle_speed: i32
}

impl Chip8 {

    pub fn new(cycle_speed :i32) -> Chip8 {
        Chip8 {
            memory: [0; 4096],
            font : [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80  // F
            ],
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            display: [false; 64*32],
            keypad: [false; 16],
            pc: 0x200,
            index_register: 0,
            registers: [0; 16],
            cycle_speed
        }
    }

    pub fn get_display(& self) -> [bool; 64*32]{
        self.display
    }

    pub fn debug_display(& self){
        let mut display :String = String::with_capacity(64*32);
        for row in 0..32{
            for col in 0..64{
                if self.display[col + (64*row)]{
                    display.push('*');
                }
                else{
                    display.push(' ');
                }
            }
            display.push('\n');
        }

        println!("{}", display);
    }

    pub fn get_cycle_speed(& self) -> i32{
        self.cycle_speed
    }

    pub fn read_input(&mut self, key_name : &str, input_state: Key){
        let mut key_num = 0;
        match key_name{
            "1" => key_num = 1,
            "2" => key_num = 2,
            "3" => key_num = 3,
            "4" => key_num = 0xC,
            "Q" => key_num = 4,
            "W" => key_num = 5,
            "E" => key_num = 6,
            "R" => key_num = 0xD,
            "A" => key_num = 7,
            "S" => key_num = 8,
            "D" => key_num = 9,
            "F" => key_num = 0xE,
            "Z" => key_num = 0xA,
            "X" => key_num = 0,
            "C" => key_num = 0xB,
            "V" => key_num = 0xF,
            _ => ()
        };
        
        // println!("Key: {} pressed", key_num);
    
        match input_state{
            Key::Pressed => self.keypad[key_num] = true,
            Key::Released => self.keypad[key_num] = false,
        }
        // println!("{}", self.keypad[key_num]);
    }

    pub fn load_rom(&mut self, path:&String){
        let mut file = File::open(path).expect("Invalid file path");

        let mut buffer : Vec<u8> = vec![];
        let _file_size = file.read_to_end(&mut buffer).expect("Error reading file");

        let start_position = 0x200;
        self.memory[start_position..(start_position+buffer.len())].copy_from_slice(&buffer);
        println!("Sucessfully loaded ROM from path {}", path);
    }

    pub fn load_font(&mut self){
        let font_start = 0x50;
        self.memory[font_start..(font_start+self.font.len())].copy_from_slice(&self.font);
    }

    fn fetch(&mut self) -> u16 {
        let instruction = ((self.memory[self.pc as usize]) as u16) << 8 | self.memory[(self.pc+1) as usize] as u16;
        self.pc+=2;
        instruction
    }

    pub fn cycle(&mut self){
        //FETCH
        let instruction :u16 = self.fetch();
        

        //DECODE+EXECUTE
        let digit_1 :u16 = (&instruction & 0xF000) >> 12;
        let digit_2 :u16 = (&instruction & 0x0F00) >> 8;
        let digit_3 :u16 = (&instruction & 0x00F0) >> 4;
        let digit_4 :u16 = &instruction & 0x000F;

        // let opcode_split: (u16, u16, u16, u16) = (digit_1, digit_2, digit_3, digit_4);

        match (digit_1, digit_2, digit_3, digit_4){
            (0,0,0,0) => (),
            //Clear screan
            (0,0,0xE,0) => self.display = [false; 64*32],
            //Jump
            (1, _, _, _) => self.pc = instruction & 0x0FFF,
            //Set register VX
            (6, _, _, _) =>{
                let nn = (instruction & 0x00FF) as u8;
                self.registers[digit_2 as usize] = nn;
            },
            //Add to register (overflow)?
            (7, _, _, _) =>{
                let nn = (instruction & 0x00FF) as u8;
                self.registers[digit_2 as usize] = self.registers[digit_2 as usize].wrapping_add(nn);
            },
            (0xA, _, _, _) =>{
                let nnn = instruction & 0x0FFF;
                self.index_register = nnn;
            },
            //DXYN
            (0xD, _, _, _) =>{
                let x_coord :i32 = (self.registers[digit_2  as usize] % 64).into();
                let mut y_coord :i32 = (self.registers[digit_3 as usize] % 32).into();
                let sprite_height = digit_4;
                self.registers[0xF] = 0;
            
                'draw_row :for i in 0..sprite_height{
                    let sprite_data_address = self.index_register + i;
                    let sprite_pixel_data :u8 = self.memory[sprite_data_address as usize];

                    for bit in (0..8).rev(){
                        let pixel_bit = (sprite_pixel_data >> bit) & 1;
                        let curr_pixel = 7-bit;
                        if (self.display[((x_coord + curr_pixel) + (64 *  y_coord)) as usize] as u8 & pixel_bit) == 1 {
                            self.display[((x_coord + curr_pixel)+ (64 * y_coord)) as usize] = false;
                            self.registers[0xF] = 1;
                        }
                        else if !self.display[((x_coord + curr_pixel) + (64*y_coord)) as usize] && (pixel_bit == 1){
                            self.display[((x_coord + curr_pixel) + (64*y_coord)) as usize] = true;
                        }
                        

                        if (x_coord) == 64{
                            break 'draw_row;
                        } 
                    }

                    y_coord +=1;
                    if y_coord == 32{
                        break 'draw_row;
                    }
                }

            }
            //Return from subroutine
            (0, 0, 0xE, 0xE) => self.pc = self.stack.pop().expect("Error popping stack value"),
            //Call subroutine
            (2, _, _, _) =>{
                self.stack.push(self.pc);
                self.pc = instruction & 0xFFF;
            },
            (3, _, _, _) =>{
                let nn = (instruction & 0x00FF) as u8;
                if self.registers[digit_2 as usize] == nn{
                    self.pc+=2;
                }
            },
            (4, _, _, _) =>{
                let nn = (instruction & 0x00FF) as u8;
                if self.registers[digit_2 as usize] != nn{
                    self.pc+=2;
                }
            },
            (5, _, _, 0) =>{
                if self.registers[digit_2 as usize] == self.registers[digit_3 as usize]{
                    self.pc+=2;
                }
            },
            (9, _, _, 0) =>{
                if self.registers[digit_2 as usize] != self.registers[digit_3 as usize]{
                    self.pc+=2;
                }
            },
            (8, _, _, 0) =>{
                self.registers[digit_2 as usize] = self.registers[digit_3 as usize];
            },
            (8, _, _, 1) =>{
                self.registers[digit_2 as usize] |= self.registers[digit_3 as usize];
            },
            (8, _, _, 2) =>{
                self.registers[digit_2 as usize] &= self.registers[digit_3  as usize];
            },
            (8, _, _, 3) =>{
                self.registers[digit_2 as usize] ^= self.registers[digit_3 as usize];
            },
            (8, _, _, 4) =>{
                let (result, overflow) = self.registers[digit_2 as usize].overflowing_add(self.registers[digit_3 as usize]);

                self.registers[digit_2 as usize] = result;

                if overflow{
                    self.registers[0xF] = 1;
                }
                else{
                    self.registers[0xF] = 0;
                }
            },
            (8, _, _, 5) =>{
                let (result, overflow) = self.registers[digit_2 as usize].overflowing_sub(self.registers[digit_3 as usize]);
            
                self.registers[digit_2 as usize] = result;
                
                if overflow{
                    self.registers[0xF] = 0;
                }
                else{
                    self.registers[0xF] = 1;
                }
            },
            (8, _, _, 7) =>{
                let (result, overflow) = self.registers[digit_3 as usize].overflowing_sub(self.registers[digit_2 as usize]);
                
                self.registers[digit_2 as usize] = result;

                if overflow{
                    self.registers[0xF] = 0;
                }
                else{
                    self.registers[0xF] = 1;
                }
            },
            (8, _, _, 6) =>{
                //Optional
                // self.registers[digit_2 as usize] = self.registers[digit_3 as usize];
                let bit = self.registers[digit_2 as usize] & 1;
                self.registers[digit_2 as usize] >>= 1;
                self.registers[0xF] = bit;
            },
            (8, _, _, 0xE) =>{
                //Optional
                // self.registers[digit_2 as usize] = self.registers[digit_3 as usize];
                let bit = self.registers[digit_2 as usize].reverse_bits() & 1;
                self.registers[digit_2 as usize] <<= 1;
                self.registers[0xF] = bit;
            },
            (0xB, _, _, _) =>{
                let nnn = instruction & 0x0FFF;
                self.pc = nnn + self.registers[0] as u16;
            },
            (0xC, _, _, _) =>{
                let nn :u8 = (instruction & 0x00FF) as u8;
                let random_num :u8 = random();
                self.registers[digit_2 as usize] = random_num & nn; 
            },
            (0xE, _, 9, 0xE) => {
                let key = self.registers[digit_2 as usize];
                if self.keypad[key as usize]{
                    self.pc+=2;
                }
            },
            (0xE, _, 0xA, 1) => {
                let key = self.registers[digit_2 as usize];
                if !self.keypad[key as usize]{
                    self.pc+=2;
                }
            },  
            (0xF, _, 0, 7) => {self.registers[digit_2 as usize] = self.delay_timer;},
            (0xF, _, 1, 5) => {self.delay_timer = self.registers[digit_2 as usize];},
            (0xF, _, 1, 8) => {self.sound_timer = self.registers[digit_2 as usize];},
            (0xF, _, 1, 0xE) => {self.index_register += (self.registers[digit_2 as usize]) as u16;},
            (0xF, _, 0, 0xA) => {
                let mut key_pressed :bool = false;
                for i in 0..self.keypad.len(){
                    if self.keypad[i]{
                        self.registers[digit_2 as usize] = i as u8;
                        key_pressed = true;
                        break;
                    }
                }

                if !key_pressed{
                    self.pc-=2;
                }
            },
            (0xF, _, 2, 9) => {
                let char = self.registers[digit_2 as usize];
                let char_index = 0x50 + (5*char);
                self.index_register =  char_index as u16;
            },
            (0xF, _, 3, 3) =>{
                let mut num = self.registers[digit_2 as usize];
                for i in 0..3{
                    self.memory[(self.index_register + (2-i)) as usize] = num%10;
                    num/=10;
                }
            },
            (0xF, _, 5, 5) =>{
                let x = digit_2 as usize;
                for i in 0..=x{
                    self.memory[(self.index_register + i as u16) as usize] = self.registers[i];
                }
            },
            (0xF, _, 6, 5) =>{
                let x = digit_2 as usize;
                for i in 0..=x{
                    self.registers[i] = self.memory[(self.index_register + i as u16) as usize];
                }
            }

            (_, _, _, _) => {   unimplemented!("Unimplemented opcode: {}", instruction)}
        }
    }

    pub fn decrement_timers(&mut self){
        if self.delay_timer > 0{
            self.delay_timer-=1;
        }

        if self.sound_timer > 0{
            self.sound_timer -=1;
        }
    }
}