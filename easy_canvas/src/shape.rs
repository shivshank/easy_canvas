use color::Rgba;
use style::Style;
use transform::Transform;

pub trait ToDrawCmd {
    fn with_state(self, transform: Transform, style: Style) -> DrawCmd;
}

macro_rules! make_shapes {
    (
        pub enum DrawCmd {
            ..,
            $(
                $cmd:ident $t:tt
            ),*,
        }

        $(
            $(#[$attr:meta])*
            pub struct $struct_form:ident aka $func_form:ident {
                $(
                    $field:ident : $field_ty:ty
                ),*,
            }
        )*
    ) => {
        pub enum DrawCmd {
            $(
                $struct_form {
                    $func_form: $struct_form,
                    style: Style,
                    transform: Transform,
                }
            ),*,

            $(
                $cmd $t
            ),*,
        }

        $(
            $(#[$attr])*
            pub struct $struct_form {
                $(
                    pub $field: $field_ty
                ),*,
            }

            impl ToDrawCmd for $struct_form {
                fn with_state(self, transform: Transform, style: Style) -> DrawCmd {
                    DrawCmd::$struct_form { $func_form: self, style, transform }
                }
            }

            #[inline]
            pub fn $func_form (
                $(
                    $field: $field_ty
                ),*,
            ) -> $struct_form {
                $struct_form {
                    $(
                        $field
                    ),*,
                }
            }
        )*
    };
}

make_shapes! {
    pub enum DrawCmd {
        ..,
        Clear(Rgba),
        UsePostProcess(String),
        Stop {},
    }

    #[doc = "A Rectangle with a given position, width, and height."]
    pub struct Rect aka rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    }

    #[doc = "A Line between two given points."]
    pub struct Line aka line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    }

    #[doc = "A Circle with a given radius around a point."]
    pub struct Circle aka circle {
        x: f32,
        y: f32,
        radius: f32,
    }

    #[doc = "An Arc struck from around a point with a given radius.

The arc is from `start_angle` to `end_angle` from the positive x-axis and goes counter
clockwise. Both angles are in radians."]
    pub struct Arc aka arc {
        x: f32,
        y: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
    }
}
