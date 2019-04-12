mod font;
mod pipeline;
mod surface;
pub mod texture;
mod types;

pub use font::Font;
pub use pipeline::Instance;
pub use surface::{winit, Surface};
pub use texture::Texture;
pub use types::{DepthView, TargetView};

use wgpu;

use crate::graphics::{Color, Transformation};
use pipeline::Pipeline;

pub struct Gpu {
    device: wgpu::Device,
    pipeline: Pipeline,
}

impl Gpu {
    pub(super) fn for_window(
        builder: winit::WindowBuilder,
        events_loop: &winit::EventsLoop,
    ) -> (Gpu, Surface) {
        let instance = wgpu::Instance::new();

        let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
            power_preference: wgpu::PowerPreference::HighPerformance,
        });

        let mut device = adapter.create_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
        });

        let pipeline = Pipeline::new(&mut device);

        let window = builder.build(events_loop).unwrap();
        let surface = Surface::new(window, &instance, &device);

        (Gpu { device, pipeline }, surface)
    }

    pub(super) fn clear(&mut self, view: &TargetView, color: Color) {
        let [r, g, b, a]: [f32; 4] = color.into();

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { todo: 0 },
        );

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &view,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color { r, g, b, a },
            }],
            depth_stencil_attachment: None,
        });

        self.device.get_queue().submit(&[encoder.finish()]);
    }

    pub(super) fn upload_texture(
        &mut self,
        image: &image::DynamicImage,
    ) -> Texture {
        Texture::new(&mut self.device, &self.pipeline, image)
    }

    pub(super) fn upload_texture_array(
        &mut self,
        layers: &[image::DynamicImage],
    ) -> Texture {
        Texture::new_array(&mut self.device, &self.pipeline, layers)
    }

    pub(super) fn create_drawable_texture(
        &mut self,
        width: u16,
        height: u16,
    ) -> texture::Drawable {
        texture::Drawable::new(&mut self.device, &self.pipeline, width, height)
    }

    pub(super) fn upload_font(&mut self, bytes: &'static [u8]) -> Font {
        Font::from_bytes(bytes)
    }

    pub(super) fn draw_texture_quads(
        &mut self,
        texture: &Texture,
        instances: &[Instance],
        view: &TargetView,
        transformation: &Transformation,
    ) {
        self.pipeline.draw_texture_quads(
            &mut self.device,
            texture.binding(),
            instances,
            &transformation,
            &view,
        );
    }

    pub(super) fn draw_font(
        &mut self,
        _font: &mut Font,
        _target: &TargetView,
        _depth: &DepthView,
    ) {
    }
}
