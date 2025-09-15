/*!
`eqx_window` provides window creation and basic rendering utilities (camera, texture, light, etc.)
There is a separate crate `eqx_render` for deeper rendering algorithms
*/

mod camera;
mod primitives;
mod texture;
mod window_display;

pub mod prelude {
    pub use crate::window_display::WindowDisplay;
}
