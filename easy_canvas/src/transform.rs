use cgmath::prelude::*;
use cgmath::{Decomposed, Basis2, Rotation2, Vector2, vec2, Transform as CgMathTransformTrait, Rad};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    decomp: Decomposed<Vector2<f32>, Basis2<f32>>
}

impl Transform {
    pub fn identity() -> Transform {
        Transform {
            decomp: Decomposed::one()
        }
    }

    pub fn from_rot<A: Angle + Into<Rad<f32>>>(angle: A) -> Transform {
        Transform {
            decomp: Decomposed {
                scale: 1.0,
                disp: vec2(0.0 , 0.0),
                rot: Rotation2::from_angle(angle)
            }
        }
    }
}