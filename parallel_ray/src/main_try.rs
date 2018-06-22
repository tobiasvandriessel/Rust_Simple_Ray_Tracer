extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::*;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator as TextureCreator1;
use std::time::Duration;

// fn main() {
//     println!("Hello, world!");
// }

const WIDTH: u32 = 800;
const HEIGHT: u32 = 512; 
 

pub fn init() -> (sdl2::Sdl, sdl2::render::Canvas<sdl2::video::Window>, TextureCreator1<sdl2::video::WindowContext>, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("raytracer", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

     let texture_creator = canvas.texture_creator();
    
        
    let mut event_pump = sdl_context.event_pump().unwrap();



    (sdl_context, canvas, texture_creator, event_pump)
}

pub fn main() {
    let (sdl_context, mut canvas, texture_creator, mut event_pump) = init();

     let mut texture = texture_creator.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, WIDTH, HEIGHT).expect("Texture");    

        'running: loop {
        match tick(&mut event_pump, &mut canvas, &mut texture){
            Ok(t) => {},
            Err(e) =>  break 'running            
        }
        
    }
}

pub fn tick(event_pump: &mut sdl2::EventPump, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, texture: &mut sdl2::render::Texture) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));        
        canvas.clear();   
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Err(String::from("Program quit"));
                },
                _ => {}
            }
        }

               // The rest of the game loop goes here...

        let res = texture.with_lock(None, |texture_buffer, pitch| {
            for p in texture_buffer.iter_mut(){
                *p = 125;
            }
        });

        canvas.copy(&texture, None, Some(Rect::new(0,0,150,150)) );


        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // if i == 255 {
        //     i = 0;
        // } else {
        //     i += 1;
        // }

        Ok(())
}

pub fn draw_texture() {
    //let mut texture =  renderer.create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32).expect("Texture");
} 
