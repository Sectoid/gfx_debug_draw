use bitmap_font::BitmapFont;
use line_renderer::LineRenderer;
use text_renderer::TextRenderer;
use image::{self, ImageFormat, DynamicImage};

use gfx::{
    Frame,
    Graphics,
    ProgramError,
    TextureHandle,
};

use gfx::traits::*;

use gfx_texture::{ Texture };

pub enum DebugRendererError {
    ShaderProgramError(ProgramError),
    BitmapFontTextureError,
}

impl From<ProgramError> for DebugRendererError {
    fn from(err: ProgramError) -> DebugRendererError {
        DebugRendererError::ShaderProgramError(err)
    }
}

pub struct DebugRenderer<D: Device, F: Factory<D::Resources>> {
    line_renderer: LineRenderer<D, F>,
    text_renderer: TextRenderer<D, F>,
}

impl<D: Device, F: Factory<D::Resources>> DebugRenderer<D, F> {

    pub fn new (
        graphics: &mut Graphics<D, F>,
        frame_size: [u32; 2],
        initial_buffer_size: usize,
        bitmap_font: Option<BitmapFont>,
        bitmap_font_texture: Option<TextureHandle<D::Resources>>,
    ) -> Result<DebugRenderer<D, F>, DebugRendererError> {

        let device_capabilities = graphics.device.get_capabilities();

        let bitmap_font = match bitmap_font {
            Some(f) => f,
            None => BitmapFont::from_string(include_str!("../assets/notosans.fnt")).unwrap()
        };

        let bitmap_font_texture = match bitmap_font_texture {
            Some(t) => t,
            None => {
                if let DynamicImage::ImageRgba8(rgba_image) = image::load_from_memory_with_format(include_bytes!("../assets/notosans.png"), ImageFormat::PNG).unwrap() {
                    Texture::from_image(&mut graphics.factory, &rgba_image, false, false).handle
                } else {
                    return Err(DebugRendererError::BitmapFontTextureError)
                }
            }
        };

        let line_renderer = try!(LineRenderer::new(*device_capabilities, &mut graphics.factory, initial_buffer_size));
        let text_renderer = try!(TextRenderer::new(*device_capabilities, &mut graphics.factory, frame_size, initial_buffer_size, bitmap_font, bitmap_font_texture));

        Ok(DebugRenderer {
            line_renderer: line_renderer,
            text_renderer: text_renderer,
        })
    }

    pub fn draw_line(&mut self, start: [f32; 3], end: [f32; 3], color: [f32; 4]) {
        self.line_renderer.draw_line(start, end, color);
    }

    pub fn draw_text_on_screen (
        &mut self,
        text: &str,
        screen_position: [i32; 2],
        color: [f32; 4],
    ) {
        self.text_renderer.draw_text_on_screen(text, screen_position, color);
    }

    pub fn draw_text_at_position (
        &mut self,
        text: &str,
        world_position: [f32; 3],
        color: [f32; 4],
    ) {
        self.text_renderer.draw_text_at_position(text, world_position, color);
    }

    pub fn render (
        &mut self,
        graphics: &mut Graphics<D, F>,
        frame: &Frame<D::Resources>,
        projection: [[f32; 4]; 4],
    ) {
        self.line_renderer.render(graphics, frame, projection);
        self.text_renderer.render(graphics, frame, projection);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.text_renderer.resize(width, height);
    }
}
