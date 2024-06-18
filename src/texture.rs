use image::io::Reader as ImageReader;
pub struct Texture<'a> {
    renderer_id: u32,
    #[allow(dead_code)]
    file_path: &'a str,
    #[allow(dead_code)]
    width: i32,
    #[allow(dead_code)]
    height: i32,
    #[allow(dead_code)]
    bpp: i32,
    // #[allow(dead_code)]
    // img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl<'a> Texture<'a> {
    pub fn new(file_path: &'a str) -> Self {
        let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageReader::open(file_path)
            .expect("Can't open file path for the image")
            .decode()
            .expect("Can't decode the image properly.")
            .flipv()
            .into_rgba8();

        let mut renderer_id = 0;
        let width = img.width() as i32;
        let height = img.height() as i32;
        let bpp = 8 * 4;
        unsafe {
            gl::GenTextures(1, &mut renderer_id);
            gl::BindTexture(gl::TEXTURE_2D, renderer_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr().cast(),
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        return Self {
            renderer_id,
            file_path,
            width,
            height,
            bpp,
            // img,
        };
    }

    pub fn bind(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.renderer_id);
        }
    }

    #[allow(dead_code)]
    pub fn undbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    #[allow(dead_code)]
    pub fn get_width(&self) -> i32 {
        self.width
    }

    #[allow(dead_code)]
    pub fn get_height(&self) -> i32 {
        self.height
    }

    #[allow(dead_code)]
    pub fn get_bpp(&self) -> i32 {
        self.bpp
    }
}

impl<'a> Drop for Texture<'a> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.renderer_id);
        }
    }
}
