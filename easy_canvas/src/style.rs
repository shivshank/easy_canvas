use color::{Color, Rgba};

#[derive(Debug, Clone, Copy)]
pub enum Style {
    FillStyle {
        color: Rgba
    },
    StrokeStyle {
        color: Rgba
    },
}

impl Style {
    #[inline]
    pub fn fill<C: Color>(color: C) -> Style {
        Style::FillStyle {
            color: color.normalize()
        }
    }
}