use color::Rgba;
use style::Style;

pub trait ToDrawCmd {
    fn with_style(self, style: Style) -> DrawCmd;
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
                }
            ),*,

            $(
                $cmd $t
            ),*,
        }

        $(
            pub struct $struct_form {
                $(
                    pub $field: $field_ty
                ),*,
            }

            impl ToDrawCmd for $struct_form {
                fn with_style(self, style: Style) -> DrawCmd {
                    DrawCmd::$struct_form { $func_form: self, style }
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
    }

    pub struct Rect aka rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    }

    pub struct Line aka line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    }

    pub struct Circle aka circle {
        x: f32,
        y: f32,
        radius: f32,
    }

    pub struct Arc aka arc {
        x: f32,
        y: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
    }
}
