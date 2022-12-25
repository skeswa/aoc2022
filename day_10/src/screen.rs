use std::fmt::Display;

/// CRT screen that updates once per cycle.
#[derive(Debug)]
pub(crate) struct Screen {
    /// How many columns of pixels this [Screen] has.
    columns: usize,
    /// Index of the next pixel to write.
    cursor: usize,
    /// Pixels on this screen.
    pixels: Vec<char>,
    /// How many pixels the sprite occupies horizaontally.
    half_sprite_width: f64,
}

impl Screen {
    /// Creates and returns a new [Screen].
    ///
    /// * `columns` is how many columns of pixels the resulting [Screen] has
    /// * `rows` is how many rows of pixels the resulting [Screen] has
    /// * `sprite_width` is how many pixels the sprite occupies horizaontally
    pub(crate) fn new(columns: usize, rows: usize, sprite_width: usize) -> Screen {
        Screen {
            columns: columns,
            cursor: 0,
            half_sprite_width: (sprite_width as f64) / 2.0,
            pixels: vec![LIT_PIXEL; columns * rows],
        }
    }

    /// Paints the current pixel, deciding what it looks like based on the value
    /// of `sprite_position` and advances to the next one.
    pub(crate) fn paint(&mut self, sprite_position: i64) {
        let sprite_displacement = ((self.cursor as i64) - sprite_position).abs() as f64;

        println!(
            "paint: {}/{} -> {}",
            sprite_position, self.cursor, sprite_displacement
        );
        let pixel = if sprite_displacement <= self.half_sprite_width {
            LIT_PIXEL
        } else {
            DARK_PIXEL
        };

        let pixel_count = self.pixels.len();
        self.pixels[self.cursor % pixel_count] = pixel;

        self.cursor += 1;
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self
            .pixels
            .chunks(self.columns)
            .map(|pixels| pixels.iter().collect::<String>());

        for line in lines {
            writeln!(f, "{}", line)?;
        }

        std::fmt::Result::Ok(())
    }
}

/// Character used to represent a unilluminated pixel.
const DARK_PIXEL: char = '.';

/// Character used to represent an illuminated pixel.
const LIT_PIXEL: char = '#';
