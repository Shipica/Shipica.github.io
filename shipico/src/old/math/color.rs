//! RGBA Colors. Many colors have predefined constants for convenience.

mod trie;

/// Describes the red, green, blue, and alpha components of a color.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Color {
    /// Red channel [0.0, 1.0]
    pub r: f64,
    /// Green channel [0.0, 1.0]
    pub g: f64,
    /// Blue channel [0.0, 1.0]
    pub b: f64,
    /// Alpha channel [0.0, 1.0]
    pub a: f64,
}

impl Color {
    /// Construct a color from its floating components
    #[inline]
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color { r, g, b, a }
    }

    /// Construct a color from its hexidecimal RGB color and floating point
    /// alpha channel. `rgb` is interpreted as `0xRRGGBB`
    #[inline]
    pub fn from_u32(rgb: u32, a: f64) -> Color {
        Color {
            r: ((rgb >> 16) & 0xFF) as f64 / 255.0,
            g: ((rgb >> 8) & 0xFF) as f64 / 255.0,
            b: (rgb & 0xFF) as f64 / 255.0,
            a,
        }
    }

    /// Linearly interpolate between two colors. `0.0` will return `self` as-is
    /// and `1.0` will return `other` as-is.
    #[inline]
    pub fn lerp(&self, other: &Color, t: f64) -> Color {
        let ti = 1.0 - t;
        Color {
            r: self.r * ti + other.r * t,
            g: self.g * ti + other.g * t,
            b: self.b * ti + other.b * t,
            a: self.a * ti + other.a * t,
        }
    }

    pub fn lookup(name: &str) -> Option<Color> {
        let mut node = &trie::NODES[0];
        let mut name = name.as_bytes();

        'iter: while !name.is_empty() {
            let idx = match name[0] {
                b @ b'a'..=b'z' => b - b'a' + 1,
                b @ b'A'..=b'Z' => b - b'A' + 1,
                _ => 0,
            };

            let s = node.children.0 as usize;
            let e = s + node.children.1 as usize;
            let list = &trie::CHILDREN[s..e];

            for &(b, nid) in list {
                if idx == b {
                    node = &trie::NODES[nid as usize];
                    name = &name[1..];
                    continue 'iter;
                }
            }

            return None;
        }

        node.color.map(|c| trie::COLORS[c as usize])
    }

    pub fn from_str_rgba(s: &str) -> Result<Color, ColorParseError> {
        match Self::from_str_raw(s) {
            ColorParseResult::BuiltinColor(color) => Ok(color),
            ColorParseResult::Data(data, false) => Ok(Color {
                r: data[0] as f64 / 255.0,
                g: data[1] as f64 / 255.0,
                b: data[2] as f64 / 255.0,
                a: 1.0,
            }),
            ColorParseResult::Data(data, true) => Ok(Color {
                r: data[0] as f64 / 255.0,
                g: data[1] as f64 / 255.0,
                b: data[2] as f64 / 255.0,
                a: data[3] as f64 / 255.0,
            }),
            ColorParseResult::ColorNotFound => Err(ColorParseError::ColorNotFound),
            ColorParseResult::BadHexFormat => Err(ColorParseError::BadHexFormat),
        }
    }

    pub fn from_str_argb(s: &str) -> Result<Color, ColorParseError> {
        match Self::from_str_raw(s) {
            ColorParseResult::BuiltinColor(color) => Ok(color),
            ColorParseResult::Data(data, false) => Ok(Color {
                r: data[0] as f64 / 255.0,
                g: data[1] as f64 / 255.0,
                b: data[2] as f64 / 255.0,
                a: 1.0,
            }),
            ColorParseResult::Data(data, true) => Ok(Color {
                a: data[0] as f64 / 255.0,
                r: data[1] as f64 / 255.0,
                g: data[2] as f64 / 255.0,
                b: data[3] as f64 / 255.0,
            }),
            ColorParseResult::ColorNotFound => Err(ColorParseError::ColorNotFound),
            ColorParseResult::BadHexFormat => Err(ColorParseError::BadHexFormat),
        }
    }

    fn from_str_raw(mut s: &str) -> ColorParseResult {
        s = s.trim();
        if let Some(color) = Color::lookup(s) {
            return ColorParseResult::BuiltinColor(color);
        }

        if s.starts_with('#') {
            s = s.trim_start_matches('#');
            if !s.chars().all(|c| c.is_digit(16)) {
                return ColorParseResult::BadHexFormat;
            }
        } else if !s.is_empty() || !s.chars().all(|c| c.is_digit(16)) {
            return ColorParseResult::ColorNotFound;
        }
        if s.len() > 8 {
            return ColorParseResult::BadHexFormat;
        }

        let data = u32::from_str_radix(s, 16).unwrap();
        let byt = |i: u32| ((data >> (i * 8)) & 0xFF) as u8;
        let nib = |i: u32| ((data >> (i * 4)) & 0xF) as u8;
        let dnib = |i| nib(i) | (nib(i) << 4);
        match s.len() {
            3 => ColorParseResult::Data([dnib(2), dnib(1), dnib(0), 0], false),
            4 => ColorParseResult::Data([dnib(3), dnib(2), dnib(1), dnib(0)], true),
            6 => ColorParseResult::Data([byt(2), byt(1), byt(0), 0], false),
            8 => ColorParseResult::Data([byt(3), byt(2), byt(1), byt(0)], true),
            _ => ColorParseResult::BadHexFormat,
        }
    }

    pub fn to_hex_string(&self) -> String {
        format!(
            "#{:#X?}{:#X?}{:#X?}{:#X?}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8
        )
    }
}

enum ColorParseResult {
    BuiltinColor(Color),
    Data([u8; 4], bool),
    ColorNotFound,
    BadHexFormat,
}

#[cfg(test)]
#[test]
fn color_lookups() {
    assert_eq!(Color::lookup("blue"), Some(Color::BLUE));
    assert_eq!(Color::lookup("BLUE"), Some(Color::BLUE));
    assert_eq!(Color::lookup("BlUe"), Some(Color::BLUE));
    assert_eq!(Color::lookup("bLuE"), Some(Color::BLUE));
    assert_eq!(Color::lookup("bluee"), None);
    assert_eq!(Color::lookup("blu"), None);

    assert_eq!(Color::lookup("alice-blue"), Some(Color::ALICE_BLUE));
    assert_eq!(Color::lookup("alice blue"), Some(Color::ALICE_BLUE));
    assert_eq!(Color::lookup("alice_blue"), Some(Color::ALICE_BLUE));
    assert_eq!(Color::lookup("alice.blue"), Some(Color::ALICE_BLUE));
    assert_eq!(Color::lookup("alice+blue"), Some(Color::ALICE_BLUE));
    assert_eq!(Color::lookup("alice-blue-"), None);
}

#[derive(Debug)]
pub enum ColorParseError {
    ColorNotFound,
    BadHexFormat,
}

impl std::fmt::Display for ColorParseError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self)
    }
}

impl std::error::Error for ColorParseError {
    fn description(&self) -> &str {
        match self {
            ColorParseError::ColorNotFound => "Color not found",
            ColorParseError::BadHexFormat => "Bad hex format",
        }
    }
}

impl std::str::FromStr for Color {
    type Err = ColorParseError;
    fn from_str(s: &str) -> Result<Color, ColorParseError> {
        Color::from_str_rgba(s)
    }
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Color::from(0)
    }
}

impl<'a> From<&'a Color> for Color {
    #[inline]
    fn from(color: &'a Color) -> Color {
        *color
    }
}

impl From<u32> for Color {
    #[inline]
    fn from(rgb: u32) -> Color {
        Color::from_u32(rgb, 1.0)
    }
}

impl From<(u32, f64)> for Color {
    #[inline]
    fn from((rgb, a): (u32, f64)) -> Color {
        Color::from_u32(rgb, a)
    }
}

// TODO: Replace this with a const fn when float ops in const fn is stable
macro_rules! define_color {
    ($r:expr, $g:expr, $b:expr) => {
        Color {
            r: $r as f64 / 255.0,
            g: $g as f64 / 255.0,
            b: $b as f64 / 255.0,
            a: 1.0,
        }
    };
}

/// <div style="background-color: #F0F8FF; width: 25px; height: 25px"></div>
pub const ALICE_BLUE: Color = define_color!(0xF0, 0xF8, 0xFF);
/// <div style="background-color: #FAEBD7; width: 25px; height: 25px"></div>
pub const ANTIQUE_WHITE: Color = define_color!(0xFA, 0xEB, 0xD7);
/// <div style="background-color: #00FFFF; width: 25px; height: 25px"></div>
pub const AQUA: Color = define_color!(0x00, 0xFF, 0xFF);
/// <div style="background-color: #7FFFD4; width: 25px; height: 25px"></div>
pub const AQUAMARINE: Color = define_color!(0x7F, 0xFF, 0xD4);
/// <div style="background-color: #F0FFFF; width: 25px; height: 25px"></div>
pub const AZURE: Color = define_color!(0xF0, 0xFF, 0xFF);
/// <div style="background-color: #F5F5DC; width: 25px; height: 25px"></div>
pub const BEIGE: Color = define_color!(0xF5, 0xF5, 0xDC);
/// <div style="background-color: #FFE4C4; width: 25px; height: 25px"></div>
pub const BISQUE: Color = define_color!(0xFF, 0xE4, 0xC4);
/// <div style="background-color: #000000; width: 25px; height: 25px"></div>
pub const BLACK: Color = define_color!(0x00, 0x00, 0x00);
/// <div style="background-color: #FFEBCD; width: 25px; height: 25px"></div>
pub const BLANCHED_ALMOND: Color = define_color!(0xFF, 0xEB, 0xCD);
/// <div style="background-color: #0000FF; width: 25px; height: 25px"></div>
pub const BLUE: Color = define_color!(0x00, 0x00, 0xFF);
/// <div style="background-color: #8A2BE2; width: 25px; height: 25px"></div>
pub const BLUE_VIOLET: Color = define_color!(0x8A, 0x2B, 0xE2);
/// <div style="background-color: #A52A2A; width: 25px; height: 25px"></div>
pub const BROWN: Color = define_color!(0xA5, 0x2A, 0x2A);
/// <div style="background-color: #DEB887; width: 25px; height: 25px"></div>
pub const BURLY_WOOD: Color = define_color!(0xDE, 0xB8, 0x87);
/// <div style="background-color: #5F9EA0; width: 25px; height: 25px"></div>
pub const CADET_BLUE: Color = define_color!(0x5F, 0x9E, 0xA0);
/// <div style="background-color: #7FFF00; width: 25px; height: 25px"></div>
pub const CHARTREUSE: Color = define_color!(0x7F, 0xFF, 0x00);
/// <div style="background-color: #D2691E; width: 25px; height: 25px"></div>
pub const CHOCOLATE: Color = define_color!(0xD2, 0x69, 0x1E);
/// <div style="background-color: #FF7F50; width: 25px; height: 25px"></div>
pub const CORAL: Color = define_color!(0xFF, 0x7F, 0x50);
/// <div style="background-color: #6495ED; width: 25px; height: 25px"></div>
pub const CORNFLOWER_BLUE: Color = define_color!(0x64, 0x95, 0xED);
/// <div style="background-color: #FFF8DC; width: 25px; height: 25px"></div>
pub const CORNSILK: Color = define_color!(0xFF, 0xF8, 0xDC);
/// <div style="background-color: #DC143C; width: 25px; height: 25px"></div>
pub const CRIMSON: Color = define_color!(0xDC, 0x14, 0x3C);
/// <div style="background-color: #00FFFF; width: 25px; height: 25px"></div>
pub const CYAN: Color = define_color!(0x00, 0xFF, 0xFF);
/// <div style="background-color: #00008B; width: 25px; height: 25px"></div>
pub const DARK_BLUE: Color = define_color!(0x00, 0x00, 0x8B);
/// <div style="background-color: #008B8B; width: 25px; height: 25px"></div>
pub const DARK_CYAN: Color = define_color!(0x00, 0x8B, 0x8B);
/// <div style="background-color: #B8860B; width: 25px; height: 25px"></div>
pub const DARK_GOLDENROD: Color = define_color!(0xB8, 0x86, 0x0B);
/// <div style="background-color: #A9A9A9; width: 25px; height: 25px"></div>
pub const DARK_GRAY: Color = define_color!(0xA9, 0xA9, 0xA9);
/// <div style="background-color: #006400; width: 25px; height: 25px"></div>
pub const DARK_GREEN: Color = define_color!(0x00, 0x64, 0x00);
/// <div style="background-color: #BDB76B; width: 25px; height: 25px"></div>
pub const DARK_KHAKI: Color = define_color!(0xBD, 0xB7, 0x6B);
/// <div style="background-color: #8B008B; width: 25px; height: 25px"></div>
pub const DARK_MAGENTA: Color = define_color!(0x8B, 0x00, 0x8B);
/// <div style="background-color: #556B2F; width: 25px; height: 25px"></div>
pub const DARK_OLIVEGREEN: Color = define_color!(0x55, 0x6B, 0x2F);
/// <div style="background-color: #FF8C00; width: 25px; height: 25px"></div>
pub const DARK_ORANGE: Color = define_color!(0xFF, 0x8C, 0x00);
/// <div style="background-color: #9932CC; width: 25px; height: 25px"></div>
pub const DARK_ORCHID: Color = define_color!(0x99, 0x32, 0xCC);
/// <div style="background-color: #8B0000; width: 25px; height: 25px"></div>
pub const DARK_RED: Color = define_color!(0x8B, 0x00, 0x00);
/// <div style="background-color: #E9967A; width: 25px; height: 25px"></div>
pub const DARK_SALMON: Color = define_color!(0xE9, 0x96, 0x7A);
/// <div style="background-color: #8FBC8F; width: 25px; height: 25px"></div>
pub const DARK_SEAGREEN: Color = define_color!(0x8F, 0xBC, 0x8F);
/// <div style="background-color: #483D8B; width: 25px; height: 25px"></div>
pub const DARK_SLATEBLUE: Color = define_color!(0x48, 0x3D, 0x8B);
/// <div style="background-color: #2F4F4F; width: 25px; height: 25px"></div>
pub const DARK_SLATEGRAY: Color = define_color!(0x2F, 0x4F, 0x4F);
/// <div style="background-color: #00CED1; width: 25px; height: 25px"></div>
pub const DARK_TURQUOISE: Color = define_color!(0x00, 0xCE, 0xD1);
/// <div style="background-color: #9400D3; width: 25px; height: 25px"></div>
pub const DARK_VIOLET: Color = define_color!(0x94, 0x00, 0xD3);
/// <div style="background-color: #FF1493; width: 25px; height: 25px"></div>
pub const DEEP_PINK: Color = define_color!(0xFF, 0x14, 0x93);
/// <div style="background-color: #00BFFF; width: 25px; height: 25px"></div>
pub const DEEP_SKYBLUE: Color = define_color!(0x00, 0xBF, 0xFF);
/// <div style="background-color: #696969; width: 25px; height: 25px"></div>
pub const DIM_GRAY: Color = define_color!(0x69, 0x69, 0x69);
/// <div style="background-color: #1E90FF; width: 25px; height: 25px"></div>
pub const DODGER_BLUE: Color = define_color!(0x1E, 0x90, 0xFF);
/// <div style="background-color: #B22222; width: 25px; height: 25px"></div>
pub const FIREBRICK: Color = define_color!(0xB2, 0x22, 0x22);
/// <div style="background-color: #FFFAF0; width: 25px; height: 25px"></div>
pub const FLORAL_WHITE: Color = define_color!(0xFF, 0xFA, 0xF0);
/// <div style="background-color: #228B22; width: 25px; height: 25px"></div>
pub const FOREST_GREEN: Color = define_color!(0x22, 0x8B, 0x22);
/// <div style="background-color: #FF00FF; width: 25px; height: 25px"></div>
pub const FUCHSIA: Color = define_color!(0xFF, 0x00, 0xFF);
/// <div style="background-color: #DCDCDC; width: 25px; height: 25px"></div>
pub const GAINSBORO: Color = define_color!(0xDC, 0xDC, 0xDC);
/// <div style="background-color: #F8F8FF; width: 25px; height: 25px"></div>
pub const GHOST_WHITE: Color = define_color!(0xF8, 0xF8, 0xFF);
/// <div style="background-color: #FFD700; width: 25px; height: 25px"></div>
pub const GOLD: Color = define_color!(0xFF, 0xD7, 0x00);
/// <div style="background-color: #DAA520; width: 25px; height: 25px"></div>
pub const GOLDENROD: Color = define_color!(0xDA, 0xA5, 0x20);
/// <div style="background-color: #808080; width: 25px; height: 25px"></div>
pub const GRAY: Color = define_color!(0x80, 0x80, 0x80);
/// <div style="background-color: #008000; width: 25px; height: 25px"></div>
pub const GREEN: Color = define_color!(0x00, 0x80, 0x00);
/// <div style="background-color: #ADFF2F; width: 25px; height: 25px"></div>
pub const GREEN_YELLOW: Color = define_color!(0xAD, 0xFF, 0x2F);
/// <div style="background-color: #F0FFF0; width: 25px; height: 25px"></div>
pub const HONEYDEW: Color = define_color!(0xF0, 0xFF, 0xF0);
/// <div style="background-color: #FF69B4; width: 25px; height: 25px"></div>
pub const HOT_PINK: Color = define_color!(0xFF, 0x69, 0xB4);
/// <div style="background-color: #CD5C5C; width: 25px; height: 25px"></div>
pub const INDIAN_RED: Color = define_color!(0xCD, 0x5C, 0x5C);
/// <div style="background-color: #4B0082; width: 25px; height: 25px"></div>
pub const INDIGO: Color = define_color!(0x4B, 0x00, 0x82);
/// <div style="background-color: #FFFFF0; width: 25px; height: 25px"></div>
pub const IVORY: Color = define_color!(0xFF, 0xFF, 0xF0);
/// <div style="background-color: #F0E68C; width: 25px; height: 25px"></div>
pub const KHAKI: Color = define_color!(0xF0, 0xE6, 0x8C);
/// <div style="background-color: #E6E6FA; width: 25px; height: 25px"></div>
pub const LAVENDER: Color = define_color!(0xE6, 0xE6, 0xFA);
/// <div style="background-color: #FFF0F5; width: 25px; height: 25px"></div>
pub const LAVENDER_BLUSH: Color = define_color!(0xFF, 0xF0, 0xF5);
/// <div style="background-color: #7CFC00; width: 25px; height: 25px"></div>
pub const LAWN_GREEN: Color = define_color!(0x7C, 0xFC, 0x00);
/// <div style="background-color: #FFFACD; width: 25px; height: 25px"></div>
pub const LEMON_CHIFFON: Color = define_color!(0xFF, 0xFA, 0xCD);
/// <div style="background-color: #ADD8E6; width: 25px; height: 25px"></div>
pub const LIGHT_BLUE: Color = define_color!(0xAD, 0xD8, 0xE6);
/// <div style="background-color: #F08080; width: 25px; height: 25px"></div>
pub const LIGHT_CORAL: Color = define_color!(0xF0, 0x80, 0x80);
/// <div style="background-color: #E0FFFF; width: 25px; height: 25px"></div>
pub const LIGHT_CYAN: Color = define_color!(0xE0, 0xFF, 0xFF);
/// <div style="background-color: #FAFAD2; width: 25px; height: 25px"></div>
pub const LIGHT_GOLDENRODYELLOW: Color = define_color!(0xFA, 0xFA, 0xD2);
/// <div style="background-color: #90EE90; width: 25px; height: 25px"></div>
pub const LIGHT_GREEN: Color = define_color!(0x90, 0xEE, 0x90);
/// <div style="background-color: #D3D3D3; width: 25px; height: 25px"></div>
pub const LIGHT_GRAY: Color = define_color!(0xD3, 0xD3, 0xD3);
/// <div style="background-color: #FFB6C1; width: 25px; height: 25px"></div>
pub const LIGHT_PINK: Color = define_color!(0xFF, 0xB6, 0xC1);
/// <div style="background-color: #FFA07A; width: 25px; height: 25px"></div>
pub const LIGHT_SALMON: Color = define_color!(0xFF, 0xA0, 0x7A);
/// <div style="background-color: #20B2AA; width: 25px; height: 25px"></div>
pub const LIGHT_SEAGREEN: Color = define_color!(0x20, 0xB2, 0xAA);
/// <div style="background-color: #87CEFA; width: 25px; height: 25px"></div>
pub const LIGHT_SKYBLUE: Color = define_color!(0x87, 0xCE, 0xFA);
/// <div style="background-color: #778899; width: 25px; height: 25px"></div>
pub const LIGHT_SLATEGRAY: Color = define_color!(0x77, 0x88, 0x99);
/// <div style="background-color: #B0C4DE; width: 25px; height: 25px"></div>
pub const LIGHT_STEELBLUE: Color = define_color!(0xB0, 0xC4, 0xDE);
/// <div style="background-color: #FFFFE0; width: 25px; height: 25px"></div>
pub const LIGHT_YELLOW: Color = define_color!(0xFF, 0xFF, 0xE0);
/// <div style="background-color: #00FF00; width: 25px; height: 25px"></div>
pub const LIME: Color = define_color!(0x00, 0xFF, 0x00);
/// <div style="background-color: #32CD32; width: 25px; height: 25px"></div>
pub const LIME_GREEN: Color = define_color!(0x32, 0xCD, 0x32);
/// <div style="background-color: #FAF0E6; width: 25px; height: 25px"></div>
pub const LINEN: Color = define_color!(0xFA, 0xF0, 0xE6);
/// <div style="background-color: #FF00FF; width: 25px; height: 25px"></div>
pub const MAGENTA: Color = define_color!(0xFF, 0x00, 0xFF);
/// <div style="background-color: #800000; width: 25px; height: 25px"></div>
pub const MAROON: Color = define_color!(0x80, 0x00, 0x00);
/// <div style="background-color: #66CDAA; width: 25px; height: 25px"></div>
pub const MEDIUM_AQUAMARINE: Color = define_color!(0x66, 0xCD, 0xAA);
/// <div style="background-color: #0000CD; width: 25px; height: 25px"></div>
pub const MEDIUM_BLUE: Color = define_color!(0x00, 0x00, 0xCD);
/// <div style="background-color: #BA55D3; width: 25px; height: 25px"></div>
pub const MEDIUM_ORCHID: Color = define_color!(0xBA, 0x55, 0xD3);
/// <div style="background-color: #9370DB; width: 25px; height: 25px"></div>
pub const MEDIUM_PURPLE: Color = define_color!(0x93, 0x70, 0xDB);
/// <div style="background-color: #3CB371; width: 25px; height: 25px"></div>
pub const MEDIUM_SEAGREEN: Color = define_color!(0x3C, 0xB3, 0x71);
/// <div style="background-color: #7B68EE; width: 25px; height: 25px"></div>
pub const MEDIUM_SLATEBLUE: Color = define_color!(0x7B, 0x68, 0xEE);
/// <div style="background-color: #00FA9A; width: 25px; height: 25px"></div>
pub const MEDIUM_SPRINGGREEN: Color = define_color!(0x00, 0xFA, 0x9A);
/// <div style="background-color: #48D1CC; width: 25px; height: 25px"></div>
pub const MEDIUM_TURQUOISE: Color = define_color!(0x48, 0xD1, 0xCC);
/// <div style="background-color: #C71585; width: 25px; height: 25px"></div>
pub const MEDIUM_VIOLETRED: Color = define_color!(0xC7, 0x15, 0x85);
/// <div style="background-color: #191970; width: 25px; height: 25px"></div>
pub const MIDNIGHT_BLUE: Color = define_color!(0x19, 0x19, 0x70);
/// <div style="background-color: #F5FFFA; width: 25px; height: 25px"></div>
pub const MINT_CREAM: Color = define_color!(0xF5, 0xFF, 0xFA);
/// <div style="background-color: #FFE4E1; width: 25px; height: 25px"></div>
pub const MISTY_ROSE: Color = define_color!(0xFF, 0xE4, 0xE1);
/// <div style="background-color: #FFE4B5; width: 25px; height: 25px"></div>
pub const MOCCASIN: Color = define_color!(0xFF, 0xE4, 0xB5);
/// <div style="background-color: #FFDEAD; width: 25px; height: 25px"></div>
pub const NAVAJO_WHITE: Color = define_color!(0xFF, 0xDE, 0xAD);
/// <div style="background-color: #000080; width: 25px; height: 25px"></div>
pub const NAVY: Color = define_color!(0x00, 0x00, 0x80);
/// <div style="background-color: #FDF5E6; width: 25px; height: 25px"></div>
pub const OLD_LACE: Color = define_color!(0xFD, 0xF5, 0xE6);
/// <div style="background-color: #808000; width: 25px; height: 25px"></div>
pub const OLIVE: Color = define_color!(0x80, 0x80, 0x00);
/// <div style="background-color: #6B8E23; width: 25px; height: 25px"></div>
pub const OLIVE_DRAB: Color = define_color!(0x6B, 0x8E, 0x23);
/// <div style="background-color: #FFA500; width: 25px; height: 25px"></div>
pub const ORANGE: Color = define_color!(0xFF, 0xA5, 0x00);
/// <div style="background-color: #FF4500; width: 25px; height: 25px"></div>
pub const ORANGE_RED: Color = define_color!(0xFF, 0x45, 0x00);
/// <div style="background-color: #DA70D6; width: 25px; height: 25px"></div>
pub const ORCHID: Color = define_color!(0xDA, 0x70, 0xD6);
/// <div style="background-color: #EEE8AA; width: 25px; height: 25px"></div>
pub const PALE_GOLDENROD: Color = define_color!(0xEE, 0xE8, 0xAA);
/// <div style="background-color: #98FB98; width: 25px; height: 25px"></div>
pub const PALE_GREEN: Color = define_color!(0x98, 0xFB, 0x98);
/// <div style="background-color: #AFEEEE; width: 25px; height: 25px"></div>
pub const PALE_TURQUOISE: Color = define_color!(0xAF, 0xEE, 0xEE);
/// <div style="background-color: #DB7093; width: 25px; height: 25px"></div>
pub const PALE_VIOLETRED: Color = define_color!(0xDB, 0x70, 0x93);
/// <div style="background-color: #FFEFD5; width: 25px; height: 25px"></div>
pub const PAPAYA_WHIP: Color = define_color!(0xFF, 0xEF, 0xD5);
/// <div style="background-color: #FFDAB9; width: 25px; height: 25px"></div>
pub const PEACH_PUFF: Color = define_color!(0xFF, 0xDA, 0xB9);
/// <div style="background-color: #CD853F; width: 25px; height: 25px"></div>
pub const PERU: Color = define_color!(0xCD, 0x85, 0x3F);
/// <div style="background-color: #FFC0CB; width: 25px; height: 25px"></div>
pub const PINK: Color = define_color!(0xFF, 0xC0, 0xCB);
/// <div style="background-color: #DDA0DD; width: 25px; height: 25px"></div>
pub const PLUM: Color = define_color!(0xDD, 0xA0, 0xDD);
/// <div style="background-color: #B0E0E6; width: 25px; height: 25px"></div>
pub const POWDER_BLUE: Color = define_color!(0xB0, 0xE0, 0xE6);
/// <div style="background-color: #800080; width: 25px; height: 25px"></div>
pub const PURPLE: Color = define_color!(0x80, 0x00, 0x80);
/// <div style="background-color: #FF0000; width: 25px; height: 25px"></div>
pub const RED: Color = define_color!(0xFF, 0x00, 0x00);
/// <div style="background-color: #BC8F8F; width: 25px; height: 25px"></div>
pub const ROSY_BROWN: Color = define_color!(0xBC, 0x8F, 0x8F);
/// <div style="background-color: #4169E1; width: 25px; height: 25px"></div>
pub const ROYAL_BLUE: Color = define_color!(0x41, 0x69, 0xE1);
/// <div style="background-color: #8B4513; width: 25px; height: 25px"></div>
pub const SADDLE_BROWN: Color = define_color!(0x8B, 0x45, 0x13);
/// <div style="background-color: #FA8072; width: 25px; height: 25px"></div>
pub const SALMON: Color = define_color!(0xFA, 0x80, 0x72);
/// <div style="background-color: #F4A460; width: 25px; height: 25px"></div>
pub const SANDY_BROWN: Color = define_color!(0xF4, 0xA4, 0x60);
/// <div style="background-color: #2E8B57; width: 25px; height: 25px"></div>
pub const SEA_GREEN: Color = define_color!(0x2E, 0x8B, 0x57);
/// <div style="background-color: #FFF5EE; width: 25px; height: 25px"></div>
pub const SEA_SHELL: Color = define_color!(0xFF, 0xF5, 0xEE);
/// <div style="background-color: #A0522D; width: 25px; height: 25px"></div>
pub const SIENNA: Color = define_color!(0xA0, 0x52, 0x2D);
/// <div style="background-color: #C0C0C0; width: 25px; height: 25px"></div>
pub const SILVER: Color = define_color!(0xC0, 0xC0, 0xC0);
/// <div style="background-color: #87CEEB; width: 25px; height: 25px"></div>
pub const SKY_BLUE: Color = define_color!(0x87, 0xCE, 0xEB);
/// <div style="background-color: #6A5ACD; width: 25px; height: 25px"></div>
pub const SLATE_BLUE: Color = define_color!(0x6A, 0x5A, 0xCD);
/// <div style="background-color: #708090; width: 25px; height: 25px"></div>
pub const SLATE_GRAY: Color = define_color!(0x70, 0x80, 0x90);
/// <div style="background-color: #FFFAFA; width: 25px; height: 25px"></div>
pub const SNOW: Color = define_color!(0xFF, 0xFA, 0xFA);
/// <div style="background-color: #00FF7F; width: 25px; height: 25px"></div>
pub const SPRING_GREEN: Color = define_color!(0x00, 0xFF, 0x7F);
/// <div style="background-color: #4682B4; width: 25px; height: 25px"></div>
pub const STEEL_BLUE: Color = define_color!(0x46, 0x82, 0xB4);
/// <div style="background-color: #D2B48C; width: 25px; height: 25px"></div>
pub const TAN: Color = define_color!(0xD2, 0xB4, 0x8C);
/// <div style="background-color: #008080; width: 25px; height: 25px"></div>
pub const TEAL: Color = define_color!(0x00, 0x80, 0x80);
/// <div style="background-color: #D8BFD8; width: 25px; height: 25px"></div>
pub const THISTLE: Color = define_color!(0xD8, 0xBF, 0xD8);
/// <div style="background-color: #FF6347; width: 25px; height: 25px"></div>
pub const TOMATO: Color = define_color!(0xFF, 0x63, 0x47);
/// <div style="background-color: #40E0D0; width: 25px; height: 25px"></div>
pub const TURQUOISE: Color = define_color!(0x40, 0xE0, 0xD0);
/// <div style="background-color: #EE82EE; width: 25px; height: 25px"></div>
pub const VIOLET: Color = define_color!(0xEE, 0x82, 0xEE);
/// <div style="background-color: #F5DEB3; width: 25px; height: 25px"></div>
pub const WHEAT: Color = define_color!(0xF5, 0xDE, 0xB3);
/// <div style="background-color: #FFFFFF; width: 25px; height: 25px"></div>
pub const WHITE: Color = define_color!(0xFF, 0xFF, 0xFF);
/// <div style="background-color: #F5F5F5; width: 25px; height: 25px"></div>
pub const WHITE_SMOKE: Color = define_color!(0xF5, 0xF5, 0xF5);
/// <div style="background-color: #FFFF00; width: 25px; height: 25px"></div>
pub const YELLOW: Color = define_color!(0xFF, 0xFF, 0x00);
/// <div style="background-color: #9ACD32; width: 25px; height: 25px"></div>
pub const YELLOW_GREEN: Color = define_color!(0x9A, 0xCD, 0x32);

impl Color {
    /// <div style="background-color: #F0F8FF; width: 25px; height: 25px"></div>
    pub const ALICE_BLUE: Color = ALICE_BLUE;
    /// <div style="background-color: #FAEBD7; width: 25px; height: 25px"></div>
    pub const ANTIQUE_WHITE: Color = ANTIQUE_WHITE;
    /// <div style="background-color: #00FFFF; width: 25px; height: 25px"></div>
    pub const AQUA: Color = AQUA;
    /// <div style="background-color: #7FFFD4; width: 25px; height: 25px"></div>
    pub const AQUAMARINE: Color = AQUAMARINE;
    /// <div style="background-color: #F0FFFF; width: 25px; height: 25px"></div>
    pub const AZURE: Color = AZURE;
    /// <div style="background-color: #F5F5DC; width: 25px; height: 25px"></div>
    pub const BEIGE: Color = BEIGE;
    /// <div style="background-color: #FFE4C4; width: 25px; height: 25px"></div>
    pub const BISQUE: Color = BISQUE;
    /// <div style="background-color: #000000; width: 25px; height: 25px"></div>
    pub const BLACK: Color = BLACK;
    /// <div style="background-color: #FFEBCD; width: 25px; height: 25px"></div>
    pub const BLANCHED_ALMOND: Color = BLANCHED_ALMOND;
    /// <div style="background-color: #0000FF; width: 25px; height: 25px"></div>
    pub const BLUE: Color = BLUE;
    /// <div style="background-color: #8A2BE2; width: 25px; height: 25px"></div>
    pub const BLUE_VIOLET: Color = BLUE_VIOLET;
    /// <div style="background-color: #A52A2A; width: 25px; height: 25px"></div>
    pub const BROWN: Color = BROWN;
    /// <div style="background-color: #DEB887; width: 25px; height: 25px"></div>
    pub const BURLY_WOOD: Color = BURLY_WOOD;
    /// <div style="background-color: #5F9EA0; width: 25px; height: 25px"></div>
    pub const CADET_BLUE: Color = CADET_BLUE;
    /// <div style="background-color: #7FFF00; width: 25px; height: 25px"></div>
    pub const CHARTREUSE: Color = CHARTREUSE;
    /// <div style="background-color: #D2691E; width: 25px; height: 25px"></div>
    pub const CHOCOLATE: Color = CHOCOLATE;
    /// <div style="background-color: #FF7F50; width: 25px; height: 25px"></div>
    pub const CORAL: Color = CORAL;
    /// <div style="background-color: #6495ED; width: 25px; height: 25px"></div>
    pub const CORNFLOWER_BLUE: Color = CORNFLOWER_BLUE;
    /// <div style="background-color: #FFF8DC; width: 25px; height: 25px"></div>
    pub const CORNSILK: Color = CORNSILK;
    /// <div style="background-color: #DC143C; width: 25px; height: 25px"></div>
    pub const CRIMSON: Color = CRIMSON;
    /// <div style="background-color: #00FFFF; width: 25px; height: 25px"></div>
    pub const CYAN: Color = CYAN;
    /// <div style="background-color: #00008B; width: 25px; height: 25px"></div>
    pub const DARK_BLUE: Color = DARK_BLUE;
    /// <div style="background-color: #008B8B; width: 25px; height: 25px"></div>
    pub const DARK_CYAN: Color = DARK_CYAN;
    /// <div style="background-color: #B8860B; width: 25px; height: 25px"></div>
    pub const DARK_GOLDENROD: Color = DARK_GOLDENROD;
    /// <div style="background-color: #A9A9A9; width: 25px; height: 25px"></div>
    pub const DARK_GRAY: Color = DARK_GRAY;
    /// <div style="background-color: #006400; width: 25px; height: 25px"></div>
    pub const DARK_GREEN: Color = DARK_GREEN;
    /// <div style="background-color: #BDB76B; width: 25px; height: 25px"></div>
    pub const DARK_KHAKI: Color = DARK_KHAKI;
    /// <div style="background-color: #8B008B; width: 25px; height: 25px"></div>
    pub const DARK_MAGENTA: Color = DARK_MAGENTA;
    /// <div style="background-color: #556B2F; width: 25px; height: 25px"></div>
    pub const DARK_OLIVEGREEN: Color = DARK_OLIVEGREEN;
    /// <div style="background-color: #FF8C00; width: 25px; height: 25px"></div>
    pub const DARK_ORANGE: Color = DARK_ORANGE;
    /// <div style="background-color: #9932CC; width: 25px; height: 25px"></div>
    pub const DARK_ORCHID: Color = DARK_ORCHID;
    /// <div style="background-color: #8B0000; width: 25px; height: 25px"></div>
    pub const DARK_RED: Color = DARK_RED;
    /// <div style="background-color: #E9967A; width: 25px; height: 25px"></div>
    pub const DARK_SALMON: Color = DARK_SALMON;
    /// <div style="background-color: #8FBC8F; width: 25px; height: 25px"></div>
    pub const DARK_SEAGREEN: Color = DARK_SEAGREEN;
    /// <div style="background-color: #483D8B; width: 25px; height: 25px"></div>
    pub const DARK_SLATEBLUE: Color = DARK_SLATEBLUE;
    /// <div style="background-color: #2F4F4F; width: 25px; height: 25px"></div>
    pub const DARK_SLATEGRAY: Color = DARK_SLATEGRAY;
    /// <div style="background-color: #00CED1; width: 25px; height: 25px"></div>
    pub const DARK_TURQUOISE: Color = DARK_TURQUOISE;
    /// <div style="background-color: #9400D3; width: 25px; height: 25px"></div>
    pub const DARK_VIOLET: Color = DARK_VIOLET;
    /// <div style="background-color: #FF1493; width: 25px; height: 25px"></div>
    pub const DEEP_PINK: Color = DEEP_PINK;
    /// <div style="background-color: #00BFFF; width: 25px; height: 25px"></div>
    pub const DEEP_SKYBLUE: Color = DEEP_SKYBLUE;
    /// <div style="background-color: #696969; width: 25px; height: 25px"></div>
    pub const DIM_GRAY: Color = DIM_GRAY;
    /// <div style="background-color: #1E90FF; width: 25px; height: 25px"></div>
    pub const DODGER_BLUE: Color = DODGER_BLUE;
    /// <div style="background-color: #B22222; width: 25px; height: 25px"></div>
    pub const FIREBRICK: Color = FIREBRICK;
    /// <div style="background-color: #FFFAF0; width: 25px; height: 25px"></div>
    pub const FLORAL_WHITE: Color = FLORAL_WHITE;
    /// <div style="background-color: #228B22; width: 25px; height: 25px"></div>
    pub const FOREST_GREEN: Color = FOREST_GREEN;
    /// <div style="background-color: #FF00FF; width: 25px; height: 25px"></div>
    pub const FUCHSIA: Color = FUCHSIA;
    /// <div style="background-color: #DCDCDC; width: 25px; height: 25px"></div>
    pub const GAINSBORO: Color = GAINSBORO;
    /// <div style="background-color: #F8F8FF; width: 25px; height: 25px"></div>
    pub const GHOST_WHITE: Color = GHOST_WHITE;
    /// <div style="background-color: #FFD700; width: 25px; height: 25px"></div>
    pub const GOLD: Color = GOLD;
    /// <div style="background-color: #DAA520; width: 25px; height: 25px"></div>
    pub const GOLDENROD: Color = GOLDENROD;
    /// <div style="background-color: #808080; width: 25px; height: 25px"></div>
    pub const GRAY: Color = GRAY;
    /// <div style="background-color: #008000; width: 25px; height: 25px"></div>
    pub const GREEN: Color = GREEN;
    /// <div style="background-color: #ADFF2F; width: 25px; height: 25px"></div>
    pub const GREEN_YELLOW: Color = GREEN_YELLOW;
    /// <div style="background-color: #F0FFF0; width: 25px; height: 25px"></div>
    pub const HONEYDEW: Color = HONEYDEW;
    /// <div style="background-color: #FF69B4; width: 25px; height: 25px"></div>
    pub const HOT_PINK: Color = HOT_PINK;
    /// <div style="background-color: #CD5C5C; width: 25px; height: 25px"></div>
    pub const INDIAN_RED: Color = INDIAN_RED;
    /// <div style="background-color: #4B0082; width: 25px; height: 25px"></div>
    pub const INDIGO: Color = INDIGO;
    /// <div style="background-color: #FFFFF0; width: 25px; height: 25px"></div>
    pub const IVORY: Color = IVORY;
    /// <div style="background-color: #F0E68C; width: 25px; height: 25px"></div>
    pub const KHAKI: Color = KHAKI;
    /// <div style="background-color: #E6E6FA; width: 25px; height: 25px"></div>
    pub const LAVENDER: Color = LAVENDER;
    /// <div style="background-color: #FFF0F5; width: 25px; height: 25px"></div>
    pub const LAVENDER_BLUSH: Color = LAVENDER_BLUSH;
    /// <div style="background-color: #7CFC00; width: 25px; height: 25px"></div>
    pub const LAWN_GREEN: Color = LAWN_GREEN;
    /// <div style="background-color: #FFFACD; width: 25px; height: 25px"></div>
    pub const LEMON_CHIFFON: Color = LEMON_CHIFFON;
    /// <div style="background-color: #ADD8E6; width: 25px; height: 25px"></div>
    pub const LIGHT_BLUE: Color = LIGHT_BLUE;
    /// <div style="background-color: #F08080; width: 25px; height: 25px"></div>
    pub const LIGHT_CORAL: Color = LIGHT_CORAL;
    /// <div style="background-color: #E0FFFF; width: 25px; height: 25px"></div>
    pub const LIGHT_CYAN: Color = LIGHT_CYAN;
    /// <div style="background-color: #FAFAD2; width: 25px; height: 25px"></div>
    pub const LIGHT_GOLDENRODYELLOW: Color = LIGHT_GOLDENRODYELLOW;
    /// <div style="background-color: #90EE90; width: 25px; height: 25px"></div>
    pub const LIGHT_GREEN: Color = LIGHT_GREEN;
    /// <div style="background-color: #D3D3D3; width: 25px; height: 25px"></div>
    pub const LIGHT_GRAY: Color = LIGHT_GRAY;
    /// <div style="background-color: #FFB6C1; width: 25px; height: 25px"></div>
    pub const LIGHT_PINK: Color = LIGHT_PINK;
    /// <div style="background-color: #FFA07A; width: 25px; height: 25px"></div>
    pub const LIGHT_SALMON: Color = LIGHT_SALMON;
    /// <div style="background-color: #20B2AA; width: 25px; height: 25px"></div>
    pub const LIGHT_SEAGREEN: Color = LIGHT_SEAGREEN;
    /// <div style="background-color: #87CEFA; width: 25px; height: 25px"></div>
    pub const LIGHT_SKYBLUE: Color = LIGHT_SKYBLUE;
    /// <div style="background-color: #778899; width: 25px; height: 25px"></div>
    pub const LIGHT_SLATEGRAY: Color = LIGHT_SLATEGRAY;
    /// <div style="background-color: #B0C4DE; width: 25px; height: 25px"></div>
    pub const LIGHT_STEELBLUE: Color = LIGHT_STEELBLUE;
    /// <div style="background-color: #FFFFE0; width: 25px; height: 25px"></div>
    pub const LIGHT_YELLOW: Color = LIGHT_YELLOW;
    /// <div style="background-color: #00FF00; width: 25px; height: 25px"></div>
    pub const LIME: Color = LIME;
    /// <div style="background-color: #32CD32; width: 25px; height: 25px"></div>
    pub const LIME_GREEN: Color = LIME_GREEN;
    /// <div style="background-color: #FAF0E6; width: 25px; height: 25px"></div>
    pub const LINEN: Color = LINEN;
    /// <div style="background-color: #FF00FF; width: 25px; height: 25px"></div>
    pub const MAGENTA: Color = MAGENTA;
    /// <div style="background-color: #800000; width: 25px; height: 25px"></div>
    pub const MAROON: Color = MAROON;
    /// <div style="background-color: #66CDAA; width: 25px; height: 25px"></div>
    pub const MEDIUM_AQUAMARINE: Color = MEDIUM_AQUAMARINE;
    /// <div style="background-color: #0000CD; width: 25px; height: 25px"></div>
    pub const MEDIUM_BLUE: Color = MEDIUM_BLUE;
    /// <div style="background-color: #BA55D3; width: 25px; height: 25px"></div>
    pub const MEDIUM_ORCHID: Color = MEDIUM_ORCHID;
    /// <div style="background-color: #9370DB; width: 25px; height: 25px"></div>
    pub const MEDIUM_PURPLE: Color = MEDIUM_PURPLE;
    /// <div style="background-color: #3CB371; width: 25px; height: 25px"></div>
    pub const MEDIUM_SEAGREEN: Color = MEDIUM_SEAGREEN;
    /// <div style="background-color: #7B68EE; width: 25px; height: 25px"></div>
    pub const MEDIUM_SLATEBLUE: Color = MEDIUM_SLATEBLUE;
    /// <div style="background-color: #00FA9A; width: 25px; height: 25px"></div>
    pub const MEDIUM_SPRINGGREEN: Color = MEDIUM_SPRINGGREEN;
    /// <div style="background-color: #48D1CC; width: 25px; height: 25px"></div>
    pub const MEDIUM_TURQUOISE: Color = MEDIUM_TURQUOISE;
    /// <div style="background-color: #C71585; width: 25px; height: 25px"></div>
    pub const MEDIUM_VIOLETRED: Color = MEDIUM_VIOLETRED;
    /// <div style="background-color: #191970; width: 25px; height: 25px"></div>
    pub const MIDNIGHT_BLUE: Color = MIDNIGHT_BLUE;
    /// <div style="background-color: #F5FFFA; width: 25px; height: 25px"></div>
    pub const MINT_CREAM: Color = MINT_CREAM;
    /// <div style="background-color: #FFE4E1; width: 25px; height: 25px"></div>
    pub const MISTY_ROSE: Color = MISTY_ROSE;
    /// <div style="background-color: #FFE4B5; width: 25px; height: 25px"></div>
    pub const MOCCASIN: Color = MOCCASIN;
    /// <div style="background-color: #FFDEAD; width: 25px; height: 25px"></div>
    pub const NAVAJO_WHITE: Color = NAVAJO_WHITE;
    /// <div style="background-color: #000080; width: 25px; height: 25px"></div>
    pub const NAVY: Color = NAVY;
    /// <div style="background-color: #FDF5E6; width: 25px; height: 25px"></div>
    pub const OLD_LACE: Color = OLD_LACE;
    /// <div style="background-color: #808000; width: 25px; height: 25px"></div>
    pub const OLIVE: Color = OLIVE;
    /// <div style="background-color: #6B8E23; width: 25px; height: 25px"></div>
    pub const OLIVE_DRAB: Color = OLIVE_DRAB;
    /// <div style="background-color: #FFA500; width: 25px; height: 25px"></div>
    pub const ORANGE: Color = ORANGE;
    /// <div style="background-color: #FF4500; width: 25px; height: 25px"></div>
    pub const ORANGE_RED: Color = ORANGE_RED;
    /// <div style="background-color: #DA70D6; width: 25px; height: 25px"></div>
    pub const ORCHID: Color = ORCHID;
    /// <div style="background-color: #EEE8AA; width: 25px; height: 25px"></div>
    pub const PALE_GOLDENROD: Color = PALE_GOLDENROD;
    /// <div style="background-color: #98FB98; width: 25px; height: 25px"></div>
    pub const PALE_GREEN: Color = PALE_GREEN;
    /// <div style="background-color: #AFEEEE; width: 25px; height: 25px"></div>
    pub const PALE_TURQUOISE: Color = PALE_TURQUOISE;
    /// <div style="background-color: #DB7093; width: 25px; height: 25px"></div>
    pub const PALE_VIOLETRED: Color = PALE_VIOLETRED;
    /// <div style="background-color: #FFEFD5; width: 25px; height: 25px"></div>
    pub const PAPAYA_WHIP: Color = PAPAYA_WHIP;
    /// <div style="background-color: #FFDAB9; width: 25px; height: 25px"></div>
    pub const PEACH_PUFF: Color = PEACH_PUFF;
    /// <div style="background-color: #CD853F; width: 25px; height: 25px"></div>
    pub const PERU: Color = PERU;
    /// <div style="background-color: #FFC0CB; width: 25px; height: 25px"></div>
    pub const PINK: Color = PINK;
    /// <div style="background-color: #DDA0DD; width: 25px; height: 25px"></div>
    pub const PLUM: Color = PLUM;
    /// <div style="background-color: #B0E0E6; width: 25px; height: 25px"></div>
    pub const POWDER_BLUE: Color = POWDER_BLUE;
    /// <div style="background-color: #800080; width: 25px; height: 25px"></div>
    pub const PURPLE: Color = PURPLE;
    /// <div style="background-color: #FF0000; width: 25px; height: 25px"></div>
    pub const RED: Color = RED;
    /// <div style="background-color: #BC8F8F; width: 25px; height: 25px"></div>
    pub const ROSY_BROWN: Color = ROSY_BROWN;
    /// <div style="background-color: #4169E1; width: 25px; height: 25px"></div>
    pub const ROYAL_BLUE: Color = ROYAL_BLUE;
    /// <div style="background-color: #8B4513; width: 25px; height: 25px"></div>
    pub const SADDLE_BROWN: Color = SADDLE_BROWN;
    /// <div style="background-color: #FA8072; width: 25px; height: 25px"></div>
    pub const SALMON: Color = SALMON;
    /// <div style="background-color: #F4A460; width: 25px; height: 25px"></div>
    pub const SANDY_BROWN: Color = SANDY_BROWN;
    /// <div style="background-color: #2E8B57; width: 25px; height: 25px"></div>
    pub const SEA_GREEN: Color = SEA_GREEN;
    /// <div style="background-color: #FFF5EE; width: 25px; height: 25px"></div>
    pub const SEA_SHELL: Color = SEA_SHELL;
    /// <div style="background-color: #A0522D; width: 25px; height: 25px"></div>
    pub const SIENNA: Color = SIENNA;
    /// <div style="background-color: #C0C0C0; width: 25px; height: 25px"></div>
    pub const SILVER: Color = SILVER;
    /// <div style="background-color: #87CEEB; width: 25px; height: 25px"></div>
    pub const SKY_BLUE: Color = SKY_BLUE;
    /// <div style="background-color: #6A5ACD; width: 25px; height: 25px"></div>
    pub const SLATE_BLUE: Color = SLATE_BLUE;
    /// <div style="background-color: #708090; width: 25px; height: 25px"></div>
    pub const SLATE_GRAY: Color = SLATE_GRAY;
    /// <div style="background-color: #FFFAFA; width: 25px; height: 25px"></div>
    pub const SNOW: Color = SNOW;
    /// <div style="background-color: #00FF7F; width: 25px; height: 25px"></div>
    pub const SPRING_GREEN: Color = SPRING_GREEN;
    /// <div style="background-color: #4682B4; width: 25px; height: 25px"></div>
    pub const STEEL_BLUE: Color = STEEL_BLUE;
    /// <div style="background-color: #D2B48C; width: 25px; height: 25px"></div>
    pub const TAN: Color = TAN;
    /// <div style="background-color: #008080; width: 25px; height: 25px"></div>
    pub const TEAL: Color = TEAL;
    /// <div style="background-color: #D8BFD8; width: 25px; height: 25px"></div>
    pub const THISTLE: Color = THISTLE;
    /// <div style="background-color: #FF6347; width: 25px; height: 25px"></div>
    pub const TOMATO: Color = TOMATO;
    /// <div style="background-color: #40E0D0; width: 25px; height: 25px"></div>
    pub const TURQUOISE: Color = TURQUOISE;
    /// <div style="background-color: #EE82EE; width: 25px; height: 25px"></div>
    pub const VIOLET: Color = VIOLET;
    /// <div style="background-color: #F5DEB3; width: 25px; height: 25px"></div>
    pub const WHEAT: Color = WHEAT;
    /// <div style="background-color: #FFFFFF; width: 25px; height: 25px"></div>
    pub const WHITE: Color = WHITE;
    /// <div style="background-color: #F5F5F5; width: 25px; height: 25px"></div>
    pub const WHITE_SMOKE: Color = WHITE_SMOKE;
    /// <div style="background-color: #FFFF00; width: 25px; height: 25px"></div>
    pub const YELLOW: Color = YELLOW;
    /// <div style="background-color: #9ACD32; width: 25px; height: 25px"></div>
    pub const YELLOW_GREEN: Color = YELLOW_GREEN;
}
