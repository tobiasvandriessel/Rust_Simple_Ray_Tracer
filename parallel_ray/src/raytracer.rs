pub mod raytracer {

    use std;
    use scene::scene::{Camera, Light};
    use primitives::primitives::{Primitive, Intersectable, Ray, Triangle}; 
    use cg::{vec3, Vector3, InnerSpace, Zero, prelude};

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 512; 

    const EPSILON: f32 = 0.0001;

    pub struct RayTracer {

    }

    impl RayTracer {
        pub fn trace_part_screen(screen_buffer: &mut [u8], primitives: &[Primitive], triangles: &Vec<Triangle>, lights: &[Light], 
                    cam: &Camera, id: u32, total: u32 ) {
            let mut end = 800 * 512 - 1;
            if total != 0 {
                end = 800 * 512 / total - 1;
            }

            // //trace here
            // for i in 0..end_y {
            //     for j in 0..end_x {
            //         let Vector3 { x: a, y: b, z: c } = RayTracer::render(primitives, triangles, lights, cam, &j, &i);
            //         screen_buffer[(i * end_x * 3 + j * 3 + 0) as usize] = (a * 255.) as u8;
            //         screen_buffer[(i * end_x * 3 + j * 3 + 1) as usize] = (b * 255.) as u8;
            //         screen_buffer[(i * end_x * 3 + j * 3 + 2) as usize] = (c * 255.) as u8;   
            //     }
            // }

            //trace here
            for i in 0..end {
                let y = i / 800 + id;
                let x = i - (i / 800 * 800) ; 
                let Vector3 { x: a, y: b, z: c } = RayTracer::render(primitives, triangles, lights, cam, &x, &y);
                screen_buffer[(i * 3 + 0) as usize] = (a * 255.) as u8;
                screen_buffer[(i * 3 + 1) as usize] = (b * 255.) as u8;
                screen_buffer[(i * 3 + 2) as usize] = (c * 255.) as u8;
            }

            //mutex lock here
            //plot to texture here
                // let part_screen = Rect::new(begin_x as i32, begin_y as i32, end_x - begin_x, end_y - begin_y);
                // let res = texture.with_lock(part_screen, |texture_buffer, pitch| {
                //     for (i, p) in texture_buffer.iter_mut().enumerate(){
                //         *p = screen_buffer[i];
                //     }
                // });
        

        }

        pub fn render(primitives: &[Primitive], triangles: &Vec<Triangle>, lights: &[Light], cam: &Camera, j: &u32, i: &u32) -> Vector3<f32> {
            let u = (*j as f32) / (WIDTH as f32);
            let v = (*i as f32) / (HEIGHT as f32);

            let p = cam.get_p0() + u * (cam.get_p1() - cam.get_p0()) + v * (cam.get_p2() - cam.get_p0());
            let d = (p - cam.get_pos()).normalize();
            let r = Ray::new(cam.get_pos(), d);

            //call Trace here 
            let color =  RayTracer::trace(primitives, triangles, lights, &r);

            //And then NormalShading on that
            RayTracer::normal_shading(color)
        }

        pub fn normal_shading(color: Vector3<f32>) -> Vector3<f32> {
            vec3(color.x.min(1.).max(0.), color.y.min(1.).max(0.), color.z.min(1.).max(0.))
            //let result_color = (((color.x * 255.) as u32) << 16) + (((color.y * 255.) as u32) << 8) + ((color.z * 255.) as u32);
        }

        pub fn trace(primitives: &[Primitive], triangles: &Vec<Triangle>, lights: &[Light], r: &Ray) -> Vector3<f32> {
            let mut id = -1;
            let mut n: Vector3<f32> = Vector3::zero();
            let mut color: Vector3<f32> = Vector3::zero();
            let mut i: Vector3<f32> = Vector3::zero();

            RayTracer::nearest_intersection(primitives, triangles, r, &mut i, &mut id, &mut n, &mut color);

            if id == -1 {
                return Vector3::zero()
            }

            prelude::ElementWise::mul_element_wise(color, RayTracer::direct_illumination(primitives, triangles, lights, &i, &n))
        }

        pub fn nearest_intersection(primitives: &[Primitive], triangles: &Vec<Triangle>, r: &Ray, I: &mut Vector3<f32>, 
                        id: &mut i32, n: &mut Vector3<f32>, color: &mut Vector3<f32>) {
            let mut t = std::f32::MAX;

            for pr in primitives.iter() {
                pr.intersect(r, &mut t, id, n, color);
            }

            for tr in triangles.iter() {
                tr.intersect(r, &mut t, id, n, color );
            }

            if *id != -1 {
                *I = r.o + t * r.d;
            }
        }

        pub fn direct_illumination(primitives: &[Primitive], triangles: &Vec<Triangle>, lights: &[Light], I: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32>{
            let mut total_illumination = vec3(0.0, 0.0, 0.0);
            for l in lights.iter() {
                let light_dir = l.get_pos() - I;
                let sqrd_dist = light_dir.magnitude2();
                let dist = sqrd_dist.sqrt();
                //let dist = light_dir.magnitude();
                let d = (1.0 / dist) * light_dir ; //normalize
                //let D = light_dir.normalize() ; //normalize                
                //let r = Ray::new(*I + std::f32::EPSILON * D, D);
                let r = Ray::new(*I + EPSILON * d, d);
                
                let mut id = -1;
                //let mut t = dist;// - std::f32::EPSILON;
                let mut t = dist - EPSILON;                
                let mut n1: Vector3<f32> = Vector3::zero();
                let mut color: Vector3<f32> = Vector3::zero();

                for pr in primitives.iter() {
                    pr.intersect(&r, &mut t, &mut id, &mut n1, &mut color);
                }

                for i in triangles.iter() {
                    i.intersect(&r, &mut t, &mut id, &mut n1, &mut color);
                }

                if id == -1 {
                    total_illumination += l.get_color() * (n.dot(d) * l.get_intensity() / sqrd_dist);
                    //total_illumination += l.get_color() * (n.dot(D) * l.get_intensity() / (dist * dist));
                    
                }
            }

            total_illumination
        }

    }
}
