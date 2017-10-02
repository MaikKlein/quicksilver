extern crate gl;

use gl::types::*;
use geom::Rectangle;
use std::os::raw::c_void;
use std::ops::Drop;

pub enum PixelFormat {
    RGB = gl::RGB as isize,
    RGBA = gl::RGBA as isize,
    BGR = gl::BGR as isize,
    BGRA = gl::BGRA as isize 
}

pub struct TextureData {
    id: u32,
    width: i32,
    height: i32
}

impl TextureData {
    pub fn from_raw(data: &[u8], w: i32, h: i32, format: PixelFormat) -> TextureData {
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture as *mut GLuint);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, w, h, 0, format as u32, 
                           gl::UNSIGNED_BYTE, data.as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            TextureData {
                id: texture,
                width: w,
                height: h,
            }
        }
    }

    pub fn region(&self) -> TextureRegion {
        TextureRegion {
            source: self,
            region: Rectangle::new_sized(self.width as f32, self.height as f32)
        }
    }
}

impl Drop for TextureData {
    fn drop(&mut self) {
        gl::DeleteTextures(1, self.id);
    }
}

pub struct TextureRegion<'a> {
    source: &'a TextureData,
    region: Rectangle
}

impl<'a> TextureRegion<'a> {
    pub fn get_id(&self) -> u32 {
        self.source.id
    }

    pub fn get_width(&self) -> i32 {
        self.source.width
    }

    pub fn get_height(&self) -> i32 {
        self.source.height
    }

    pub fn get_region(&self) -> Rectangle {
        self.region
    }

    pub fn subregion(&self, rect: Rectangle) -> TextureRegion {
        TextureRegion {
            source: self.source,
            region: Rectangle::new(self.region.x + rect.x, self.region.y + rect.y,
                                   rect.width, rect.height)
        }
    }
}
