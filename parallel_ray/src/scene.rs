pub mod scene {

    use cg::{vec3, Vector3, InnerSpace}; 

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 512; 

    pub struct Camera {
        pos: Vector3<f32>,
        view_direction: Vector3<f32>,
        d: f32,
        screen_center: Vector3<f32>,
        screen_corner0: Vector3<f32>,
        screen_corner1: Vector3<f32>,
        screen_corner2: Vector3<f32>,
        up: Vector3<f32>,
        right: Vector3<f32>,
        ratio: f32
    }

    impl Camera {
        pub fn new(pos: Vector3<f32>, view_direction: Vector3<f32>, d: f32) -> Camera{
            let ratio = (WIDTH as f32) / (HEIGHT as f32);
            let up = vec3(0.0, -1.0, 0.0);
            let right = vec3(1.0, 0.0, 0.0);
            let screen_center = pos + d * view_direction;
            let screen_corner0 = screen_center - ratio * right + up;
            let screen_corner1 = screen_center + ratio * right + up;
            let screen_corner2 = screen_center - ratio * right - up;
            
            Camera {
                pos, view_direction, d, screen_center, screen_corner0, screen_corner1, screen_corner2, up, right, ratio
            }
        }

        fn calculate_set_screen(&mut self){
            self.screen_center = self.pos + self.d * self.view_direction;            
            self.screen_corner0 = self.screen_center - self.ratio * self.right + self.up;
            self.screen_corner1 = self.screen_center + self.ratio * self.right + self.up;
            self.screen_corner2 = self.screen_center - self.ratio * self.right - self.up;
        }


        pub fn set_pos_rot(&mut self, pos: Vector3<f32>, view_direction: Vector3<f32>, up: Vector3<f32>, right: Vector3<f32>) {
            self.pos = pos;
            self.view_direction = view_direction;
            self.up = up;
            self.right = right;
            self.calculate_set_screen();
        }

        pub fn move_camera(&mut self, forward: bool, positive: f32){
            if forward {
                self.pos += 0.25 * positive * self.view_direction;
                self.calculate_set_screen();
            } else {
                self.pos += 0.25 * positive * self.right;
                self.calculate_set_screen();
            }
            
        }

        pub fn turn_camera(&mut self, up: bool, positive: f32) {
            if up {
                self.view_direction = ((self.screen_center + 0.1 * positive * self.up) - self.pos).normalize();
                self.up = self.right.cross(self.view_direction).normalize();

                self.calculate_set_screen();
            } else {
                self.view_direction = ((self.screen_center + 0.1 * positive * self.right) - self.pos).normalize();
                self.right = self.view_direction.cross(self.up).normalize();

                self.calculate_set_screen();
            }
        }

        pub fn get_pos(&self) -> Vector3<f32> {
            self.pos
        }

        pub fn get_view_direction(&self) -> Vector3<f32> {
            self.view_direction
        }

        pub fn get_p0(&self) -> Vector3<f32> {
            self.screen_corner0
        }

        pub fn get_p1(&self) -> Vector3<f32> {
            self.screen_corner1
        }

        pub fn get_p2(&self) -> Vector3<f32> {
            self.screen_corner2
        }

        pub fn get_up(&self) -> Vector3<f32> {
            self.up
        }

        pub fn get_right(&self) -> Vector3<f32> {
            self.right
        }

    }


    pub struct Light {
        pos: Vector3<f32>,
        intensity: f32,
        color: Vector3<f32>
    }

    impl Light {
        pub fn new(pos: Vector3<f32>, intensity: f32, color: Vector3<f32> ) -> Light {
            Light {
                pos, intensity, color
            }
        }

        pub fn get_pos(&self) -> Vector3<f32> {
            self.pos
        }

        pub fn get_intensity(&self) -> f32 {
            self.intensity
        }

        pub fn get_color(&self) -> Vector3<f32> {
            self.color
        }
    }

}
