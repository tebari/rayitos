use crate::vector::Vector3;

pub fn color_float_to_u8(color: f64) -> u8 {
    (color * 255.00) as u8
}

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b }
    }

    pub fn from(vector: Vector3) -> Pixel {
        Pixel::new(
            color_float_to_u8(vector.x()),
            color_float_to_u8(vector.y()),
            color_float_to_u8(vector.z()),
        )
    }

    pub fn get_red(&self) -> u8 {
        self.r
    }

    pub fn get_green(&self) -> u8 {
        self.g
    }

    pub fn get_blue(&self) -> u8 {
        self.b
    }
}

fn init_bitmap(width: u32, height: u32) -> Vec<Pixel> {
    let capacity = width * height;
    let mut bitmap: Vec<Pixel> = Vec::with_capacity(capacity as usize);
    for _i in 0..capacity {
        bitmap.push(Pixel::new(0, 0, 0))
    }
    bitmap
}

pub struct Image {
    width: u32,
    height: u32,
    pixelmap: Vec<Pixel>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            pixelmap: init_bitmap(width, height),
        }
    }

    pub fn from_tiles(width: u32, height: u32, mut tiles: Vec<Tile>) -> Image {
        // Sort tiles by starting line
        tiles.sort_by(|t1, t2| t1.start_x().cmp(&t2.start_x()));

        let capacity = width * height;
        let mut pixelmap: Vec<Pixel> = Vec::with_capacity(capacity as usize);

        tiles.reverse();
        while let Some(tile) = tiles.pop() {
            let mut pixel = tile.extract_image().pixelmap;
            pixelmap.append(&mut pixel);
        }

        Image {
            width,
            height,
            pixelmap,
        }
    }

    fn xy_to_index(&self, x: u32, y: u32) -> usize {
        (x * self.width + y) as usize
    }

    pub fn set(&mut self, x: u32, y: u32, pixel: Pixel) {
        let index = self.xy_to_index(x, y);
        self.pixelmap[index] = pixel
    }

    pub fn get(&self, x: u32, y: u32) -> &Pixel {
        self.pixelmap.get(self.xy_to_index(x, y)).unwrap()
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_pixel_count(&self) -> u32 {
        self.width * self.height
    }
}

impl IntoIterator for Image {
    type Item = Pixel;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixelmap.into_iter()
    }
}

pub struct Tile {
    start_x: u32,
    start_y: u32,
    image: Image,
}

impl Tile {
    pub fn new(start_x: u32, start_y: u32, width: u32, height: u32) -> Tile {
        Tile {
            start_x,
            start_y,
            image: Image::new(width, height),
        }
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn set(&mut self, x: u32, y: u32, pixel: Pixel) {
        self.image.set(x - self.start_x, y - self.start_y, pixel);
    }

    pub fn start_x(&self) -> u32 {
        self.start_x
    }

    pub fn start_y(&self) -> u32 {
        self.start_y
    }

    pub fn extract_image(self) -> Image {
        self.image
    }
}
