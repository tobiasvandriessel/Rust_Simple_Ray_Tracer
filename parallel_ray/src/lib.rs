extern crate sdl2;
extern crate cgmath as cg;
extern crate tobj;
extern crate time;
extern crate scoped_pool;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::*;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
//use std::time::Duration;
use cg::{vec3}; 
//use primitives::primitives::Primitive;
use scene::scene::{Camera, Light};
use primitives::primitives::{Primitive, Triangle, Sphere, Plane}; 
use raytracer::raytracer::RayTracer;
//use threadpool::threadpool::ThreadPool;
//use scene::scene
use std::path::Path;
use scoped_pool::Pool;
//use tobj;
//use primitives;
//use na::{Vector3, Rotation3};   

pub mod primitives;
pub mod scene;
pub mod raytracer;
pub mod threadpool;

const THREADS_NR: usize = 8;
const JOBS_NR: u32 = 512;
const FRAMES_NR: u32 = 1024;
//bulshit

//const primitives_nr: u32 = 8;


 
 pub fn main(width: u32, height: u32) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("raytracer", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, width, height).expect("Texture");

    let cube_translation = vec3(-4., -1., 1.);

    let cube = tobj::load_obj(&Path::new("./assets/cube.obj"));
    assert!(cube.is_ok());
    let (models, _materials) = cube.unwrap();

    let mut ind = 0;
    let mut triangles = Vec::new();

    for (_i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        for f in 0..mesh.indices.len() / 3 {    
            let p0 = vec3(mesh.positions[(mesh.indices[3 * f + 0] * 3) as usize],  mesh.positions[(mesh.indices[3 * f + 0] * 3 + 1) as usize], mesh.positions[(mesh.indices[3 * f + 0] * 3 + 2) as usize])
                    + cube_translation; 
            let p1 = vec3(mesh.positions[(mesh.indices[3 * f + 1] * 3) as usize],  mesh.positions[(mesh.indices[3 * f + 1] * 3 + 1) as usize], mesh.positions[(mesh.indices[3 * f + 1] * 3 + 2) as usize])
                    + cube_translation; 
            let p2 = vec3(mesh.positions[(mesh.indices[3 * f + 2] * 3) as usize],  mesh.positions[(mesh.indices[3 * f + 2] * 3 + 1) as usize], mesh.positions[(mesh.indices[3 * f + 2] * 3 + 2) as usize])
                    + cube_translation; 
            let n = vec3(mesh.normals[(mesh.indices[3 * f + 0] * 3) as usize], mesh.normals[(mesh.indices[3 * f + 0] * 3 + 1) as usize], mesh.normals[(mesh.indices[3 * f + 0] * 3 + 2) as usize]);

            // println!("Triangle {:?} indices: p0: ({:?}, {:?}, {:?}), p1: ({:?}, {:?}, {:?}), p2: ({:?}, {:?}, {:?}) ", 
            //             ind, 
            //             mesh.indices[3 * f + 0], mesh.indices[3 * f + 0] + 1, mesh.indices[3 * f + 0] + 2, 
            //             mesh.indices[3 * f + 1], mesh.indices[3 * f + 1] + 1, mesh.indices[3 * f + 1] + 2,
            //             mesh.indices[3 * f + 2], mesh.indices[3 * f + 2] + 1, mesh.indices[3 * f + 2] + 2);
            //De indices kloppen gewoon
            // println!("Triangle {:?} indices: {:?}, {:?}, {:?} ", 
            //             ind, 
            //             mesh.indices[3 * f + 0],// mesh.indices[3 * f + 0] + 1, mesh.indices[3 * f + 0] + 2, 
            //             mesh.indices[3 * f + 1],// mesh.indices[3 * f + 1] + 1, mesh.indices[3 * f + 1] + 2,
            //             mesh.indices[3 * f + 2]);//, mesh.indices[3 * f + 2] + 1, mesh.indices[3 * f + 2] + 2);
            // println!("Triangle vectors {:?}: p0: ({:?}, {:?}, {:?}), p1: ({:?}, {:?}, {:?}), p2: ({:?}, {:?}, {:?}), n: ({:?}, {:?}, {:?})  ", 
            //             ind, p0.x, p0.y, p0.z, p1.x, p1.y, p1.z, p2.x, p2.y, p2.z, n.x, n.y, n.z );
            
            triangles.push(primitives::primitives::Triangle::new(ind, vec3(255.0, 0.0, 0.0), 
                p0, p1, p2, n) );
            ind += 1;
        }
        assert!(mesh.positions.len() % 3 == 0);
        assert!(mesh.normals.len() % 3 == 0);
    }

    // for i in triangles.iter() {
    // }

    //tobj::print_model_info(&models, &materials);

    let plane0 = Primitive::P(Plane::new(ind, vec3(1., 0.5, 0.5), vec3(5., 2., 0.), vec3(0., -1., 0.)));
    ind += 1;    
    let plane1 = Primitive::P(Plane::new(ind, vec3(1., 1., 1.), vec3(0., 0., 7.), vec3(0., 0., -1.)));
    ind += 1;    
    let plane2 = Primitive::P(Plane::new(ind, vec3(0.5, 1., 0.7), vec3(0., 0., -30.), vec3(0., 0., 1.)));
    ind += 1;    
    let plane3 = Primitive::P(Plane::new(ind, vec3(0.5, 1., 0.7), vec3(-15., 0., 0.), vec3(1., 0., 0.)));
    ind += 1;    
    let plane4 = Primitive::P(Plane::new(ind, vec3(0.5, 1., 0.7), vec3(15., 0., 0.), vec3(-1., 0., 0.)));
    ind += 1;    
    let plane5 = Primitive::P(Plane::new(ind, vec3(0.5, 1., 0.7), vec3(0., -20., 0.), vec3(0., 1., 0.)));
    ind += 1;    

    let sphere0 = Primitive::S(Sphere::new(ind, vec3(1., 1., 1.), vec3(4., -0.5, 1.), 0.25));
    ind += 1;    
    let sphere1 = Primitive::S(Sphere::new(ind, vec3(0., 0., 1.), vec3(1., -3., 1.), 0.25));
    ind += 1;        
    
    let primitives = [plane0, plane1, plane2, plane3, plane4, plane5, sphere0, sphere1];


    let mut cam = scene::scene::Camera::new(vec3(0.0, -1.0, -4.0), vec3(0.0, 0.0, 1.0) , 1.0);
    let light1 = scene::scene::Light::new(vec3(-10.0, -15.0, -8.0), 250.0, vec3(1.0, 1.0, 1.0));
    let light2 = scene::scene::Light::new(vec3(10.0, -15.0, -8.0), 250.0, vec3(1.0, 1.0, 1.0));
    
    let lights = [light1, light2];

    // let mut jobs: Vec<f32> = vec!(1.);
    // //jobs.pop
    // let jobs_handler = Arc::new(Mutex::new(jobs));

    let pool = Pool::new(THREADS_NR);
    //let thread_pool = ThreadPool::new(8);

    //let mut threads = [0, 1, 2, 3, 4, 5, 6, 7];   

    // let join_handles = (0..8).map(|i| { 
    //     let jobs_handler = Arc::clone(&jobs_handler);        
    //     thread::spawn(move || {
    //         loop {
    //             match jobs_handler.lock().unwrap().pop() {
    //                 None => {},
    //                 Some(num) => println!("num: {}",num )
    //             }
    //         }
    //         //println!("num: {}", num[0]);
    //     }) 
    // } );

    // join_handles.for_each(|i| {
    //     i.join();
    // });
    
    //let mut screen_buffer = [0; 800 * 512 * 3];
    let mut screen_buffer: Box<[u8]> = Box::new([0; 800 * 512 * 3]);

    let mut event_pump = sdl_context.event_pump().unwrap();
    //'running: loop {
    //    match handle_input(&mut event_pump, &mut cam) {
    //        Ok(_) => {},
    //        Err(_) => break 'running
    //    }
    //    tick(&pool, &mut canvas, &mut texture, &mut screen_buffer, &primitives, &triangles, &lights, &cam, true);
    //    
    //}
    
 run_timings(&mut event_pump, &mut cam, &pool, &mut canvas, &mut texture, &mut screen_buffer, &primitives, &triangles, &lights, true);
    

    


    

 }

pub fn run_timings(event_pump: &mut sdl2::EventPump, cam: &mut Camera, pool: &Pool, canvas: &mut Canvas<Window>, texture: &mut Texture, screen_buffer: &mut Box<[u8]>, primitives: &[Primitive], triangles: &Vec<Triangle>, lights: &[Light], mt: bool){
   
    run_timing( event_pump,  cam, pool,  canvas,  texture,  screen_buffer, primitives, triangles, lights, mt);
    

    cam.move_camera(true, 10.0);


    run_timing( event_pump,  cam, pool,  canvas,  texture,  screen_buffer, primitives, triangles, lights, mt);
    
    cam.move_camera(false, -20.0);
    cam.move_camera(true, 5.0);

    run_timing( event_pump,  cam, pool,  canvas,  texture,  screen_buffer, primitives, triangles, lights, mt);
    cam.turn_camera(false, 30.0);


    run_timing( event_pump,  cam, pool,  canvas,  texture,  screen_buffer, primitives, triangles, lights, mt);
    cam.turn_camera(false, 5.0);
    cam.move_camera(false, -5.0);

    run_timing( event_pump,  cam, pool,  canvas,  texture,  screen_buffer, primitives, triangles, lights, mt);
    
    cam.set_pos_rot( vec3(5., 0., -0.), vec3(-1., 0., 0.), vec3(0., -1., 0.), vec3(0.,  0., 1.));
    //cam.move_camera(true, 50.0);
    //cam.turn_camera(false, 65.0);


    run_timing( event_pump,  cam, pool,  canvas,  texture,  screen_buffer, primitives, triangles, lights, mt);


} 


pub fn run_timing( event_pump: &mut sdl2::EventPump, cam: &mut Camera, pool: &Pool, canvas: &mut Canvas<Window>, texture: &mut Texture, screen_buffer: &mut Box<[u8]>, primitives: &[Primitive], triangles: &Vec<Triangle>, lights: &[Light], mt: bool){
  
    println!("New Timing");
    let mut timing_array = [0;FRAMES_NR as usize];
    
    match handle_input(event_pump, cam) {
            Ok(_) => {},
            Err(_) => {}
        }

    timing_array[0] = tick(pool, canvas, texture, screen_buffer, primitives, triangles, lights, cam, mt);

    'running: for i in 1..(FRAMES_NR as usize){
        match handle_input(event_pump, cam) {
            Ok(_) => {},
            Err(_) => break 'running
        }
       timing_array[i] = tick(pool, canvas, texture, screen_buffer, primitives, triangles, lights, cam, mt);
    }

    for i in 0..(FRAMES_NR as usize){
        println!("{}", timing_array[i]);
    }

}



pub fn tick(pool: &Pool, canvas: &mut Canvas<Window>, texture: &mut Texture, screen_buffer: &mut Box<[u8]>, primitives: &[Primitive], triangles: &Vec<Triangle>, lights: &[Light], cam: &Camera, mt: bool) -> u64 {
    let t0 = time::precise_time_ns();
    
    canvas.set_draw_color(Color::RGB(0, 0, 0));        
    canvas.clear();   
    

            // The rest of the game loop goes here...



    //Call RayTracer::trace_part_screen here    

    //let mut thread_arr = [thread::JoinHandle; 8];
    if mt{
        pool.scoped(|scope| { 
            let chunks_enum = screen_buffer.chunks_mut((800 * 3 * 512 / JOBS_NR) as usize).enumerate();        
            chunks_enum.for_each(|(i, chunk)| {
                scope.execute(move || {
                    RayTracer::trace_part_screen(chunk, primitives, triangles, lights, cam, i as u32, JOBS_NR);                
                })
            } );
        });
    } else {
        RayTracer::trace_part_screen(screen_buffer, primitives, triangles, lights, cam, 0, 0);
    }

    let part_screen = Rect::new(0, 0, 800, 512); //begin_x as i32, begin_y as i32, end_x - begin_x, end_y - begin_y);
    let _res = texture.with_lock(part_screen, |texture_buffer, _pitch| {
        for (i, p) in texture_buffer.iter_mut().enumerate(){
            *p = screen_buffer[i];
        }
    });
    //chunks.next();
    // thread::spawn(move |chunks.next()| {
        
    // });
    //RayTracer::trace_part_screen(screen_buffer, primitives, triangles, lights, cam, texture, 0, 0, 0);

    canvas.copy(&texture, None, None);
    canvas.present();

    //let t1 = time::precise_time_ns();

    //returns time in microsecons
    (time::precise_time_ns() - t0) / 1000

    //println!("Tick cost {} miliseconds", (t1 - t0) / 1000000 );
    //let diff = (t1 - t0);

}


 pub fn handle_input(event_pump: &mut sdl2::EventPump, cam: &mut Camera) -> Result<(), String> {
     for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Err(String::from("Program quit"));
                },
                Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    cam.move_camera(true, 1.0);
                },
                Event::KeyDown {keycode: Some(Keycode::S), .. } => {
                    cam.move_camera(true, -1.0);
                },
                Event::KeyDown {keycode: Some(Keycode::A), .. } => {
                    cam.move_camera(false, -1.0);
                },
                Event::KeyDown {keycode: Some(Keycode::D), .. } => {
                    cam.move_camera(false, 1.0);
                },
                Event::KeyDown {keycode: Some(Keycode::Up), .. } => {
                    cam.turn_camera(true, 1.0);
                },
                Event::KeyDown {keycode: Some(Keycode::Down), .. } => {
                    cam.turn_camera(true, -1.0);
                },
                Event::KeyDown {keycode: Some(Keycode::Left), .. } => {
                    cam.turn_camera(false, -1.0);
                },
                Event::KeyDown {keycode: Some(Keycode::Right), .. } => {
                    cam.turn_camera(false, 1.0);
                },
                _ => {}
            }
        }

        Ok(())
 }
