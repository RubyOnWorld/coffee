use crate::graphics::gpu::{self, texture, Gpu};
use crate::graphics::{Quad, Target};
use crate::load::Task;
use crate::Result;

/// An off-screen rendering target.
///
/// It can be used both as a [`Target`] and as a resource.
///
/// [`Target`]: struct.Target.html
#[derive(Clone)]
pub struct Canvas {
    drawable: texture::Drawable,
}

impl Canvas {
    /// Create a new [`Canvas`] with the given size.
    ///
    /// [`Canvas`]: struct.Canvas.html
    pub fn new(gpu: &mut Gpu, width: u16, height: u16) -> Result<Canvas> {
        Ok(Canvas {
            drawable: gpu.create_drawable_texture(width, height),
        })
    }

    /// Create a [`Task`] that produces a new [`Canvas`] with the given size.
    ///
    /// [`Task`]: ../load/struct.Task.html
    /// [`Canvas`]: struct.Canvas.html
    pub fn load(width: u16, height: u16) -> Task<Canvas> {
        Task::using_gpu(move |gpu| Canvas::new(gpu, width, height))
    }

    /// View the [`Canvas`] as a [`Target`].
    ///
    /// [`Canvas`]: struct.Canvas.html
    /// [`Target`]: struct.Target.html
    pub fn as_target<'a>(&mut self, gpu: &'a mut Gpu) -> Target<'a> {
        let texture = self.drawable.texture();

        Target::with_transformation(
            gpu,
            self.drawable.target().clone(),
            texture.width() as f32,
            texture.height() as f32,
            texture::Drawable::render_transformation(),
        )
    }

    /// Render the [`Canvas`] on the given [`Target`].
    ///
    /// [`Canvas`]: struct.Canvas.html
    /// [`Target`]: struct.Target.html
    pub fn draw(&self, quad: Quad, target: &mut Target) {
        target.draw_texture_quads(
            &self.drawable.texture(),
            &[gpu::Instance::from(quad)],
        );
    }
}
