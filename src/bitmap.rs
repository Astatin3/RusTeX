use icy_sixel::{sixel_string, DiffusionMethod, MethodForLargest, MethodForRep, PixelFormat, Quality};

pub struct Bitmap {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width*height],
            width,
            height
        }
    }

    pub fn from_data(data: Vec<u8>, width: usize, height: usize) -> Self {
        assert!(data.len() == width * height);
        Self {
            data,
            width,
            height
        }
    }

    pub fn overlay(&mut self, other: &Bitmap, xoffset: usize, yoffset:usize) {
        for y in 0..other.height {
            for x in 0..other.width {
                self.data[(y+yoffset as usize)*self.width + (x+xoffset as usize)] = other.data[y*other.width + x];
            }
        }
    }

    /// Prints a 1 byte per pixel greyscale bitmap to Sixel format in console
    pub fn print(&self) {
        let mut bitmap_rgb888 = vec![0; self.width*self.height*3];

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y*self.width + x;

                let pixel = self.data[index];

                bitmap_rgb888[index*3] = pixel;
                bitmap_rgb888[index*3 + 1] = pixel;
                bitmap_rgb888[index*3 + 2] = pixel;
            }
        }
        
        let sixel_data = sixel_string(
            &bitmap_rgb888,
            self.width as i32,
            self.height as i32,
            PixelFormat::RGB888,
            DiffusionMethod::None,
            MethodForLargest::Auto,
            MethodForRep::Auto,
            Quality::AUTO,
        ).unwrap();

        println!("{}", sixel_data);

    }
}