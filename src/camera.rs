use nalgebra as na;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: na::Matrix4<f32> = na::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    pub position: na::Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new(position: na::Vector3<f32>, yaw: f32, pitch: f32) -> Self {
        Self {
            position,
            yaw,
            pitch,
        }
    }

    pub fn calculate_matrix(&self) -> na::Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        look_to_rh(
            &self.position.into(),
            &na::Vector3::new(
                cos_pitch * cos_yaw,
                sin_pitch,
                cos_pitch * sin_yaw,
            ),
            &na::Vector3::y(),
        )
    }
}

pub fn look_to_rh<S>(eye: &na::Point3<S>, dir: &na::Vector3<S>, up: &na::Vector3<S>) -> na::Matrix4<S>
where S:
    Clone + na::Scalar + na::SimdComplexField
{
    let f = dir.normalize();
    let s = f.cross(&up).normalize();
    let u = s.cross(&f);

    na::Matrix4::new(
        s.x.clone(), s.y.clone(), s.z.clone(), -eye.coords.dot(&s),
        u.x.clone(), u.y.clone(), u.z.clone(), -eye.coords.dot(&u),
        -f.x.clone(), -f.y.clone(), -f.z.clone(), eye.coords.dot(&f),
        S::zero(), S::zero(), S::zero(), S::one(),
    )
}

pub struct Projection {
    aspect: f32,
    fovy: f32,
    z_near: f32,
    z_far: f32,
}

impl Projection {
    pub fn new(width: u32, height: u32, fovy: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy,
            z_near,
            z_far,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calculate_matrix(&self) -> na::Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * na::Matrix4::new_perspective(
            self.aspect,
            self.fovy,
            self.z_near,
            self.z_far,
        )
    }
}
