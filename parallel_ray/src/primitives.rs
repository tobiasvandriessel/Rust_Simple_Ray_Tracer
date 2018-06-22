//use na::{Vector3, Rotation3};

pub mod primitives {

    use std;
    use cg::{Vector3, InnerSpace}; 

    //use na::Vector3;
    
    pub enum Primitive {
        S(Sphere),
        P(Plane)
    }

    impl Primitive{
        pub fn intersect(&self, r: &Ray, t: &mut f32, id: &mut i32, n: &mut Vector3<f32>, color: &mut Vector3<f32>){
            match *self {
                Primitive::S(ref s) => {
                    s.intersect(r, t, id, n, color);
                },
                Primitive::P(ref p) => {
                    p.intersect(r, t, id, n, color);
                }
            }
        }
    }

    pub trait Intersectable{
        fn get_id(&self) -> &i32;
        fn get_color(&self) -> &Vector3<f32>;
        fn intersect(&self, r: &Ray, t: &mut f32, id: &mut i32, n: &mut Vector3<f32>, color: &mut Vector3<f32>);
    }

    pub struct Ray {
        pub o: Vector3<f32>,
        pub d: Vector3<f32>
    }

    impl Ray {
        pub fn new(o: Vector3<f32>, d: Vector3<f32>) -> Ray {
            Ray {
                o,
                d
            }
        }
    }


    pub struct Triangle {
        id: i32,
        color: Vector3<f32>,
        p0: Vector3<f32>,
        p1: Vector3<f32>,
        p2: Vector3<f32>,
        n:  Vector3<f32>
    }

    impl Triangle {
        pub fn new(id: i32, color: Vector3<f32>, p0: Vector3<f32>, p1: Vector3<f32>, p2: Vector3<f32>, n: Vector3<f32>) -> Triangle {
            Triangle { id, color, p0, p1, p2, n  }
        }
    }

    impl Intersectable for Triangle {
        fn get_id(&self) -> &i32 {
            &self.id
        }

        fn get_color(&self) -> &Vector3<f32> {
            &self.color
        }

        fn intersect(&self, r: &Ray, t: &mut f32, id: &mut i32, n: &mut Vector3<f32>, color: &mut Vector3<f32>) {        
            let e1 = self.p1 - self.p0;
            let e2 = self.p2 - self.p0;
            let p = r.d.cross(e2);
            let det = e1.dot(p);
            if det > -std::f32::EPSILON && det < std::f32::EPSILON { return; }

            let inv_det = 1.0 / det;
            let T = r.o - self.p0;
            let u = T.dot(p) * inv_det;
            if u < 0. || u > 1. { return; }
            let Q = T.cross(e1);
            let v = r.d.dot(Q) * inv_det;
            if v < 0. || u + v > 1. { return; }
            let new_t = e2.dot(Q) * inv_det;
            if new_t > std::f32::EPSILON && new_t < *t {
                *t = new_t;
                *id = self.id;
                *n = self.n;
                *color = self.color;
            }
        }
    }

    pub struct Sphere {
        id: i32,
        pos: Vector3<f32>,
        color: Vector3<f32>,
        r2: f32
    }

    impl Sphere {
        pub fn new(id: i32, color: Vector3<f32>, pos: Vector3<f32>, r2: f32) -> Sphere {
            Sphere {
                id, pos, color, r2
            }
        }
    }

    impl Intersectable for Sphere {
        fn get_id(&self) -> &i32 {
            &self.id
        }

        fn get_color(&self) -> &Vector3<f32> {
            &self.color
        }

        fn intersect(&self, r: &Ray, t: &mut f32, id: &mut i32, n: &mut Vector3<f32>, color: &mut Vector3<f32>){
            let a = r.d.dot(r.d);
            let b = (2. * r.d).dot(r.o - self.pos);
            let c = (r.o - self.pos).dot(r.o - self.pos) - self.r2;

            let d = (b * b) - (4. * a * c);
            if d < 0. { return }

            let d_sqrt = d.sqrt();
            let inv_2a = 1. / (2. * a);
            let plus_t = (-b + d_sqrt) * inv_2a;
            let minus_t = (-b - d_sqrt) * inv_2a;

            let temp_t;
            if plus_t >= 0. && plus_t < minus_t {
                temp_t = plus_t;
            } else {
                temp_t = minus_t;
            }

            if temp_t >= 0. && temp_t < *t {
                *t = temp_t;
                *id = self.id;
                *n = (r.o + *t * r.d - self.pos).normalize();
                *color = self.color;
            }
            
        }
    }

    pub struct Plane {
        id: i32,
        color: Vector3<f32>,
        n: Vector3<f32>,
        d: f32    
    }

    impl Plane {
        pub fn new(id: i32, color: Vector3<f32>, p0: Vector3<f32>, n: Vector3<f32>) -> Plane {
            let n_normalized = n.normalize();
            let d = -1. * n.dot(p0);
            Plane {
                id, color, n: n_normalized, d
            }
        }
    }

    impl Intersectable for Plane {
        fn get_id(&self) -> &i32 {
            &self.id
        }

        fn get_color(&self) -> &Vector3<f32> {
            &self.color
        }

        fn intersect(&self, r: &Ray, t: &mut f32, id: &mut i32, n: &mut Vector3<f32>, color: &mut Vector3<f32>){
            let denom = r.d.dot(self.n);
            if denom.abs() < 0.0001 { return; }

            let temp_t = -((r.o.dot(self.n) + self.d) / denom);
            if temp_t >= 0. && temp_t < *t {
                *t = temp_t;
                *id = self.id;
                *n = self.n;
                *color = self.color;
            }
        }
    }




}   
