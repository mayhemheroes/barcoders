//! Functionality for generating SVG representations of barcodes.
//!
//! An SVG can be constructed via the standard constructor pattern
//! or via a constructor method if you want default values.
//!
//! For example:
//!
//! ```rust
//! use barcoders::generators::svg::*;
//!
//! // Specify your own struct fields.
//! let svg = SVG{height: 80,
//!               xdim: 1};
//!
//! // Or use the constructor for defaults (you must specify the height).
//! let svg = SVG::new(100);
//! ```

use error::Result;

/// Represents a RGBA color for the barcode foreground and background.
#[derive(Copy, Clone, Debug)]
pub struct Color {
    /// Reg, Green, Blue, Alpha value.
    rgba: [u8; 4],
}

impl Color {
    /// Constructor.
    pub fn new(rgba: [u8; 4]) -> Color {
        Color{rgba: rgba}
    }

    /// Constructor for black (#000000).
    pub fn black() -> Color {
        Color::new([0, 0, 0, 255])
    }

    /// Constructor for white (#FFFFFF).
    pub fn white() -> Color {
        Color::new([255, 255, 255, 255])
    }

    fn to_hex(&self) -> String {
        format!("#{}{}{}", self.part_to_hex(self.rgba[0]), self.part_to_hex(self.rgba[1]), self.part_to_hex(self.rgba[2]))
    }

    fn part_to_hex(&self, n: u8) -> String {
        let first = match n / 16 {
            d if d < 10 => (d + 48) as char,
            d if d < 16 => (d + 87) as char,
            _ => '0',
        };

        let second = match n % 16 {
            d if d < 10 => (d + 48) as char,
            d if d < 16 => (d + 87) as char,
            _ => '0',
        };

        format!("{}{}", first, second)
    }

    fn to_opacity(&self) -> String {
        format!("{:.*}", 2, (self.rgba[3] / 255))
    }
}

/// The SVG barcode generator type.
#[derive(Copy, Clone, Debug)]
pub struct SVG {
    /// The height of the barcode (```self.height``` pixels high for SVG).
    pub height: u32,
    /// The X dimension. Specifies the width of the "narrow" bars. 
    /// For SVG, each will be ```self.xdim``` pixels wide.
    pub xdim: u32,
    /// The RGBA color for the foreground.
    pub foreground: Color,
    /// The RGBA color for the foreground.
    pub background: Color,
}

impl SVG {
    /// Returns a new SVG with default values.
    pub fn new(height: u32) -> SVG {
        SVG {
            height: height,
            xdim: 1,
            foreground: Color{rgba: [0, 0, 0, 255]},
            background: Color{rgba: [255, 255, 255, 255]},
        }
    }

    fn rect(&self, style: u8, offset: u32, width: u32) -> String {
        let fill = match style {
            1 => self.foreground,
            _ => self.background,
        };

        let opacity = match &fill.to_opacity()[..] {
            "1.00" | "1" => "".to_string(),
            o => format!(" fill-opacity=\"{}\" ", o),
        };

        format!("<rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\"{}/>",
                offset, width, self.height, fill.to_hex(), opacity)
    }

    /// Generates the given barcode. Returns a `Result<String, Error>` of the SVG data or an
    /// error message.
    pub fn generate<T: AsRef<[u8]>>(&self, barcode: T) -> Result<String> {
        let barcode = barcode.as_ref();
        let width = (barcode.len() as u32) * self.xdim;
        let rects: String = barcode.iter()
            .enumerate()
            .filter(|&(_, &n)| n == 1)
            .map(|(i, &n)| self.rect(n, (i as u32 * self.xdim), self.xdim))
            .collect();

        Ok(format!("<svg version=\"1.1\" viewBox=\"0 0 {w} {h}\">{s}{r}</svg>",
                   w=width, h=self.height, s=self.rect(0, 0, width), r=rects))
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    use ::sym::ean8::*;
    use ::sym::code39::*;
    use ::sym::code128::*;
    use ::sym::ean_supp::*;
    use ::sym::tf::*;
    use ::sym::codabar::*;
    use ::generators::svg::*;
    use std::io::prelude::*;
    use std::io::BufWriter;
    use std::fs::File;
    use std::path::Path;

    const TEST_DATA_BASE: &str = "./target/debug";
    const WRITE_TO_FILE: bool = true;

    fn write_file(data: &str, file: &'static str) {
        let path = open_file(file);
        let mut writer = BufWriter::new(path);
        writer.write(data.as_bytes()).unwrap();
    }

    fn open_file(name: &'static str) -> File {
        File::create(&Path::new(&format!("{}/{}", TEST_DATA_BASE, name)[..])).unwrap()
    }

    #[test]
    fn ean_13_as_svg() {
        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let svg = SVG::new(80);
        let generated = svg.generate(&ean13.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "ean13.svg"); }

        assert_eq!(generated.len(), 2840);
    }

    #[test]
    fn colored_ean_13_as_svg() {
        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let svg = SVG{height: 80,
                      xdim: 1,
                      background: Color{rgba: [255, 0, 0, 255]},
                      foreground: Color{rgba: [0, 0, 255, 255]}};
        let generated = svg.generate(&ean13.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "ean13_colored.svg"); }

        assert_eq!(generated.len(), 2840);
    }

    #[test]
    fn ean_8_as_svg() {
        let ean8 = EAN8::new("9998823".to_owned()).unwrap();
        let svg = SVG::new(80);
        let generated = svg.generate(&ean8.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "ean8.svg"); }

        assert_eq!(generated.len(), 1888);
    }

    #[test]
    fn code39_as_svg() {
        let code39 = Code39::new("IGOT99PROBLEMS".to_owned()).unwrap();
        let svg = SVG::new(80);
        let generated = svg.generate(&code39.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "code39.svg"); }

        assert_eq!(generated.len(), 6426);
    }

    #[test]
    fn codabar_as_svg() {
        let codabar = Codabar::new("A12----34A".to_owned()).unwrap();
        let svg = SVG::new(80);
        let generated = svg.generate(&codabar.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "codabar.svg"); }

        assert_eq!(generated.len(), 2899);
    }

    #[test]
    fn code128_as_svg() {
        let code128 = Code128::new("ÀHIĆ345678".to_owned()).unwrap();
        let svg = SVG::new(80);
        let generated = svg.generate(&code128.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "code128.svg"); }

        assert_eq!(generated.len(), 2676);
    }

    #[test]
    fn ean_2_as_svg() {
        let ean2 = EANSUPP::new("78".to_owned()).unwrap();
        let svg = SVG::new(80);
        let generated = svg.generate(&ean2.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "ean2.svg"); }

        assert_eq!(generated.len(), 713);
    }

    #[test]
    fn itf_as_svg() {
        let itf = TF::interleaved("1234123488993344556677118".to_owned()).unwrap();
        let svg = SVG{height: 80,
                      xdim: 1,
                      background: Color::black(),
                      foreground: Color::white()};
        let generated = svg.generate(&itf.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "itf.svg"); }

        assert_eq!(generated.len(), 7161);
    }
}
