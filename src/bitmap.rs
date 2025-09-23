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


impl Bitmap {
    /// Sets a pixel value with bounds checking
    fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
        }
    }

    /// Gets a pixel value with bounds checking
    fn get_pixel(&self, x: usize, y: usize) -> u8 {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            0
        }
    }


    /// Draws an antialiased line with arbitrary thickness
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, thickness: f32, color: u8) {
        let x0 = x0 as f32;
        let y0 = y0 as f32;
        let x1 = x1 as f32;
        let y1 = y1 as f32;

        let dx = x1 - x0;
        let dy = y1 - y0;
        let length = (dx * dx + dy * dy).sqrt();
        
        if length < 0.001 {
            // Handle degenerate case of zero-length line
            // self.draw_thick_point(x0 as usize, y0 as usize, thickness, color);
            return;
        }

        // Unit vector perpendicular to the line
        // let perp_x = -dy / length;
        // let perp_y = dx / length;

        // Half thickness for calculations
        let half_thickness = thickness * 0.5;

        // Calculate bounding box with some padding for antialiasing
        let padding = (thickness * 0.5 + 1.0).ceil() as i32;
        let min_x = ((x0.min(x1) - padding as f32).floor() as i32).max(0) as usize;
        let max_x = ((x0.max(x1) + padding as f32).ceil() as i32).min(self.width as i32 - 1) as usize;
        let min_y = ((y0.min(y1) - padding as f32).floor() as i32).max(0) as usize;
        let max_y = ((y0.max(y1) + padding as f32).ceil() as i32).min(self.height as i32 - 1) as usize;

        // For each pixel in the bounding box, calculate distance to line
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let px = x as f32;
                let py = y as f32;

                // Calculate distance from point to line segment
                let distance = self.point_to_line_segment_distance(px, py, x0, y0, x1, y1);
                
                // Calculate alpha based on distance and thickness
                let alpha = self.calculate_alpha(distance, half_thickness);
                
                if alpha > 0.0 {
                    self.blend_pixel(x, y, color, alpha);
                }
            }
        }
    }

    /// Blends a pixel with the existing value using alpha blending
    fn blend_pixel(&mut self, x: usize, y: usize, color: u8, alpha: f32) {
        if x < self.width && y < self.height {
            let existing = self.get_pixel(x, y) as f32;
            let new_value = (existing * (1.0 - alpha) + color as f32 * alpha).round() as u8;
            self.set_pixel(x, y, new_value);
        }
    }


    /// Calculates the shortest distance from a point to a line segment
    fn point_to_line_segment_distance(&self, px: f32, py: f32, x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let length_sq = dx * dx + dy * dy;

        if length_sq < 0.001 {
            // Line segment is actually a point
            let dpx = px - x0;
            let dpy = py - y0;
            return (dpx * dpx + dpy * dpy).sqrt();
        }

        // Calculate parameter t for the closest point on the line segment
        let t = ((px - x0) * dx + (py - y0) * dy) / length_sq;
        let t = t.max(0.0).min(1.0); // Clamp to [0, 1] to stay on segment

        // Find the closest point on the line segment
        let closest_x = x0 + t * dx;
        let closest_y = y0 + t * dy;

        // Return distance to closest point
        let dpx = px - closest_x;
        let dpy = py - closest_y;
        (dpx * dpx + dpy * dpy).sqrt()
    }
    
    /// Calculates alpha value based on distance from line edge
    fn calculate_alpha(&self, distance: f32, half_thickness: f32) -> f32 {
        if distance <= half_thickness - 0.5 {
            // Inside the line core - full opacity
            1.0
        } else if distance <= half_thickness + 0.5 {
            // In the antialiasing zone - linear falloff
            half_thickness + 0.5 - distance
        } else {
            // Outside the line - transparent
            0.0
        }
    }


    // // Draw a thick line by creating a capsule shape
    // fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, width: usize, color: u8) {
    //     let half_width = width as f32 / 2.0;

    //     // Calculate line vector and length
    //     let dx = (x2 - x1) as f32;
    //     let dy = (y2 - y1) as f32;
    //     let length = (dx * dx + dy * dy).sqrt();

    //     // if length < 0.001 {
    //     //     // Degenerate case: just draw a circle
    //     //     self.draw_circle(x1, y1, half_width as i32, color);
    //     //     return;
    //     // }

    //     // Normalize the line direction
    //     let nx = dx / length;
    //     let ny = dy / length;

    //     // Calculate perpendicular vector for line thickness
    //     let px = -ny * half_width;
    //     let py = nx * half_width;

    //     // Get the four corners of the rectangle
    //     let corner1_x = (x1 as f32 + px) as usize;
    //     let corner1_y = (y1 as f32 + py) as usize;
    //     let corner2_x = (x1 as f32 - px) as usize;
    //     let corner2_y = (y1 as f32 - py) as usize;
    //     let corner3_x = (x2 as f32 - px) as usize;
    //     let corner3_y = (y2 as f32 - py) as usize;
    //     let corner4_x = (x2 as f32 + px) as usize;
    //     let corner4_y = (y2 as f32 + py) as usize;

    //     // Draw the main rectangle body
    //     self.fill_quadrilateral(
    //         corner1_x, corner1_y, corner2_x, corner2_y, corner3_x, corner3_y, corner4_x, corner4_y,
    //         color,
    //     );

    //     // // Draw rounded ends
    //     // let radius = (half_width) as i32;
    //     // self.draw_circle(x1, y1, radius, color);
    //     // self.draw_circle(x2, y2, radius, color);
    // }

    // fn fill_quadrilateral(
    //     &mut self,
    //     x1: usize,
    //     y1: usize,
    //     x2: usize,
    //     y2: usize,
    //     x3: usize,
    //     y3: usize,
    //     x4: usize,
    //     y4: usize,
    //     color: u8,
    // ) {
    //     // Find bounding box
    //     let binding_x = [x1, x2, x3, x4];
    //     let binding_y = [y1, y2, y3, y4];
    //     let min_x = binding_x.iter().min().unwrap();
    //     let max_x = binding_x.iter().max().unwrap();
    //     let min_y = binding_y.iter().min().unwrap();
    //     let max_y = binding_y.iter().max().unwrap();

    //     // For each point in bounding box, test if it's inside the quadrilateral
    //     for y in *min_y..=*max_y {
    //         for x in *min_x..=*max_x {
    //             if self.point_in_quad(x, y, x1, y1, x2, y2, x3, y3, x4, y4) {
    //                 self.set_pixel(x, y, color);
    //             }
    //         }
    //     }
    // }

    // // Test if a point is inside a quadrilateral using cross products
    // fn point_in_quad(
    //     &self,
    //     px: usize,
    //     py: usize,
    //     x1: usize,
    //     y1: usize,
    //     x2: usize,
    //     y2: usize,
    //     x3: usize,
    //     y3: usize,
    //     x4: usize,
    //     y4: usize,
    // ) -> bool {
    //     // Test against each edge of the quadrilateral
    //     let sign1 = self.cross_product(px - x1, py - y1, x2 - x1, y2 - y1);
    //     let sign2 = self.cross_product(px - x2, py - y2, x3 - x2, y3 - y2);
    //     let sign3 = self.cross_product(px - x3, py - y3, x4 - x3, y4 - y3);
    //     let sign4 = self.cross_product(px - x4, py - y4, x1 - x4, y1 - y4);

    //     // Point is inside if all cross products have the same sign
    //     (sign1 >= 0 && sign2 >= 0 && sign3 >= 0 && sign4 >= 0)
    //         || (sign1 <= 0 && sign2 <= 0 && sign3 <= 0 && sign4 <= 0)
    // }

    // // Calculate 2D cross product
    // fn cross_product(&self, ax: usize, ay: usize, bx: usize, by: usize) -> usize {
    //     ax * by - ay * bx
    // }

    // fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
    //     self.data[y*self.width + x] = color;
    // }
}