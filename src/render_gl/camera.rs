use glm::*;
use render_gl::to_radians;

pub struct Camera {
    position: Vec3,
    front: Vec3,
    up: Vec3,
    speed: f32,
    yaw: f32,
    pitch: f32
}
pub enum CameraMovement {
    Left,
    Right,
    Front,
    Back
}

impl Camera {
    // front implies the delta from the camera that we want to point to,
    // i.e; front = position.z + front.z and not the exact point that we point to
    // (which is controlled by moving the camera)
    pub fn new(pos: &Vec3, front: &Vec3, u: &Vec3, speed: f32) -> Camera {
        Camera {
            position: pos.clone(),
            front: front.clone(),
            up: u.clone(),
            speed: speed,
            yaw: -90.0,
            pitch: 0.0,
        }
    }
    
    pub fn look_at(&self) -> Mat4 {
       look_at(&self.position, &&(self.front + self.position), &self.up)
     }

    pub fn position(&self) -> Vec3 {
        self.position.clone()
    }

    pub fn update_angle(&mut self, pitch_offset: f32, yaw_offset: f32) {
        self.pitch += pitch_offset;
        self.yaw   += yaw_offset;
        let g = |x| { if x < 0.0 { 0.0} else if x > 89.9  { 89.9} else {x}};
        self.pitch = g(self.pitch);
        self.yaw   = g(self.yaw);
        let mut new_vec = make_vec3(&[0.0, 0.0, 0.0]);
        new_vec.x = to_radians(self.yaw).cos()  * to_radians(self.pitch).cos();
        new_vec.y = to_radians(self.pitch).cos();
        new_vec.z = to_radians(self.yaw).sin() * to_radians(self.pitch).cos();
        new_vec = normalize(&new_vec);
        self.front = new_vec;
        let right = normalize(&self.front.cross(&self.up));
        self.up = normalize(&right.cross(&self.front));
        self.look_at();
    }

    pub fn update_movement(&mut self, m: CameraMovement) -> Mat4 {
        match m {
            CameraMovement::Front => {
                self.position += self.speed * self.front;
                self.look_at()
            }
            CameraMovement::Back => {
                self.position -= self.speed * self.front;
                self.look_at()
            }
            CameraMovement::Left => {
                self.position -=
                        normalize(&self.front.cross(&self.up)) * self.speed;
                self.look_at()
            }
            CameraMovement::Right => {
                self.position +=
                    normalize(&self.front.cross(&self.up)) * self.speed;
                self.look_at()
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = make_vec3(&[0.0, 3.0, 5.0]);
        let g = make_vec3(&[0.0, 0.0, 0.0]);
        let m = make_vec3(&[0.0, 1.0, 0.0]);
        let cam = Camera::new(&v, &g, &m, 3.0);
        assert_eq!(cam.position, make_vec3(&[0.0, 3.0, 5.0]));
    }
}
