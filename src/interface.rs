extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
use sdl2::rect::Rect;
use crate::Chip8;




pub struct Interface{
    window_title :String,
    window_width :u32,
    window_height :u32,
    window_scale: u32,
}

impl Interface{

    pub fn new(title :String, width: u32, height: u32, scale: u32) -> Interface{
            Interface { window_title: title, window_width: width, window_height: height, window_scale: scale}
    }

    pub fn draw(&self, emulator :&Chip8, canvas :&mut Canvas<Window>) -> Result<(), String>{
       

        canvas.set_draw_color(Color::RGB(255,0,255));
        
        
        let display = emulator.get_display();

        for col in 0..self.window_width{
            for row in 0..self.window_height{
                if display[(col + (64 * row)) as usize]{
                    let rect = Rect::new((col*self.window_scale) as i32, (row*self.window_scale) as i32, self.window_scale, self.window_scale);
                    canvas.draw_rect(rect)?;
                    canvas.fill_rect(rect)?;
                }
            }
        }
        canvas.present();
        Ok(())
    }

    pub fn window_loop(&self, emulator :&mut Chip8) -> Result<(), String>{
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(&self.window_title, self.window_width*self.window_scale, self.window_height*self.window_scale)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;


        
        
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        let mut event = sdl_context.event_pump()?;

        'running: loop {
            emulator.cycle();
            self.draw(emulator, &mut canvas)?;
            
            for event in event.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
        
}


