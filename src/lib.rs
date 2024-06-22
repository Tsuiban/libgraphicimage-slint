use slint::{Image, Rgb8Pixel, SharedPixelBuffer};

pub struct GraphicImage {
    data : SharedPixelBuffer<Rgb8Pixel>,
    current_x : u32,
    current_y : u32,
}

impl GraphicImage {
    pub fn new(width : u32, height : u32) -> GraphicImage {
        GraphicImage {
            data : SharedPixelBuffer::new(width, height),
            current_x : 0,
            current_y : 0,
        }
    }

    pub fn set_pixel(&mut self, position : (u32, u32), color: Rgb8Pixel) {
        let (x, y) = position;
        if x >= self.data.width() || y >= self.data.height() { return };
        let inverted_y = self.data.height() - y - 1;
        let y_index = inverted_y * self.data.width();
        let index = y_index + x;
        self.data.make_mut_slice()[index as usize] = color;
    }

    /*
    Equation of a line:
        y = mx + b
        where
            m = delta_y / delta_x
            b = (given point (h,x)): k-mh
     */
    pub fn line_from_to(&mut self, start: (u32, u32), end: (u32, u32), color: Rgb8Pixel) {
        let (start_x, start_y) = start;
        let (end_x, end_y) = end;
        if start_x == end_x {
            for i in start_y.min(end_y)..start_y.max(end_y) {
                self.set_pixel((start_x, i), color);
            }
        } else {
            let delta_y = end_y as i32 - start_y as i32;
            let delta_x = end_x as i32 - start_x as i32;
            let m = delta_y as f32 / delta_x as f32;
            let b = start_y as f32 - m * start_x as f32;
            for x in start_x.min(end_x)..start_x.max(end_x) {
                let y = (m * x as f32 + b).round();
                self.set_pixel((x, y as u32), color);
            }
        }
        self.current_x = end_x;
        self.current_y = end_y;
    }

    pub fn line_to(&mut self, end: (u32, u32), color : Rgb8Pixel) {
        self.line_from_to((self.current_x, self.current_y), end, color);
    }

    /*
    Equation of a circle given center at (h,k) is
    (x-h)^2 + (y-k)^2 = r^2

    Solving for y:
        (y-k)^2 = r^2 - (x-h)^2
        (y-k) = (r^2 - (x-h)^2).sqrt()
        y = (r^2 - (x-h)^2).sqrt() + k;
     */
    pub fn circle(&mut self, position: (u32, u32), radius: f32, color: Rgb8Pixel) {
        let (x, y) = position;
        let min_x = (x as f32 - radius) as i32;
        let min_x_clipped = min_x.max(0) as u32;
        let max_x = (x as f32 + radius).round() as u32;
        let max_x_clipped = max_x.min(self.data.width());
        let radius_squared = radius * radius;
        let position = (position.0 as f32, position.1 as f32);
        for i in min_x_clipped..max_x_clipped {
            let min_x = i as f32 - 0.5;
            let max_x = i as f32 + 0.5;
            let (min_y_positive, min_y_negative) = calculate_y(min_x, position, radius_squared);
            let (max_y_positive, max_y_negative) = calculate_y(max_x, position, radius_squared);
            self.line_from_to((i, min_y_positive.round() as u32), (i, max_y_positive.round() as u32), color);
            self.line_from_to((i, min_y_negative.round() as u32), (i, max_y_negative.round() as u32), color);
        }
        self.current_x = x;
        self.current_y = y;
    }

    pub fn to_image(&self) -> Image {
        Image::from_rgb8(self.data.clone())
    }
}

fn calculate_y(x : f32, position: (f32, f32), radius_squared : f32) -> (f32, f32) {
    /*
    (y-k)^2 + (x-h)^2 = r^2
    (y-k)^2 = r^2 - (x-h)^2
    (y-k) = (r^2 - (x-h)^2).sqrt()
    y = (r^2 - (x-h)^2).sqrt() + k
     */
    let delta_y = (radius_squared - (x-position.0) * (x-position.0)).sqrt();
    ((position.1 + delta_y).round(), (position.1 - delta_y).round())
}
