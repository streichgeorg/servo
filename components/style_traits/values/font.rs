/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Some font related types moved here from the style crate, so gfx can use them
//! without depending on style.

use Atom;
use app_units::Au;
use std::fmt;
use std::fmt::Write;
#[cfg(feature = "servo")] use servo_url::ServoUrl;
use super::{CssWriter, ToCss};

/// As of CSS Fonts Module Level 3, only the following values are
/// valid: 100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900
///
/// However, system fonts may provide other values. Pango
/// may provide 350, 380, and 1000 (on top of the existing values), for example.
#[derive(Clone, Copy, Debug, Eq, Hash, MallocSizeOf, PartialEq)]
#[cfg_attr(feature = "servo", derive(Deserialize, Serialize))]
pub struct FontWeight(pub u16);

impl FontWeight {
    /// Value for normal
    pub fn normal() -> Self {
        FontWeight(400)
    }

    /// Value for bold
    pub fn bold() -> Self {
        FontWeight(700)
    }

    /// Convert from an integer to Weight
    pub fn from_int(n: i32) -> Result<Self, ()> {
        if n >= 100 && n <= 900 && n % 100 == 0 {
            Ok(FontWeight(n as u16))
        } else {
            Err(())
        }
    }

    /// Convert from an Gecko weight
    pub fn from_gecko_weight(weight: u16) -> Self {
        // we allow a wider range of weights than is parseable
        // because system fonts may provide custom values
        FontWeight(weight)
    }

    /// Wether this weight is bold
    pub fn is_bold(&self) -> bool {
        self.0 > 500
    }

    /// Return the bolder weight
    pub fn bolder(self) -> Self {
        if self.0 < 400 {
            FontWeight(400)
        } else if self.0 < 600 {
            FontWeight(700)
        } else {
            FontWeight(900)
        }
    }

    /// Returns the lighter weight
    pub fn lighter(self) -> Self {
        if self.0 < 600 {
            FontWeight(100)
        } else if self.0 < 800 {
            FontWeight(400)
        } else {
            FontWeight(700)
        }
    }
}

impl ToCss for FontWeight {
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result where W: Write {
        write!(dest, "{}", self.0)
    }
}

define_css_keyword_enum! {
    pub enum FontStretch {
        Normal = "normal",
        UltraCondensed = "ultra-condensed",
        ExtraCondensed = "extra-condensed",
        Condensed = "condensed",
        SemiCondensed = "semi-condensed",
        SemiExpanded = "semi-expanded",
        Expanded = "expanded",
        ExtraExpanded = "extra-expanded",
        UltraExpanded = "ultra-expanded",
    }
}

#[cfg(feature = "servo")]
define_css_keyword_enum! {
    pub enum FontVariantCaps {
        Normal = "normal",
        SmallCaps = "small-caps",
    }
}

/// We should treat font stretch as real number in order to interpolate this property.
/// <https://drafts.csswg.org/css-fonts-3/#font-stretch-animation>
impl From<FontStretch> for f64 {
    fn from(stretch: FontStretch) -> f64 {
        use self::FontStretch::*;
        match stretch {
            UltraCondensed => 1.0,
            ExtraCondensed => 2.0,
            Condensed => 3.0,
            SemiCondensed => 4.0,
            Normal => 5.0,
            SemiExpanded => 6.0,
            Expanded => 7.0,
            ExtraExpanded => 8.0,
            UltraExpanded => 9.0,
        }
    }
}

impl Into<FontStretch> for f64 {
    fn into(self) -> FontStretch {
        use values::font::FontStretch::*;
        let index = (self + 0.5).floor().min(9.0).max(1.0);
        static FONT_STRETCH_ENUM_MAP: [FontStretch; 9] =
            [ UltraCondensed, ExtraCondensed, Condensed, SemiCondensed, Normal,
              SemiExpanded, Expanded, ExtraExpanded, UltraExpanded ];
        FONT_STRETCH_ENUM_MAP[(index - 1.0) as usize]
    }
}

/// Everything gfx needs from `style::style_structs::Font`
#[cfg(feature = "servo")]
pub trait FontStyleStruct {
    /// Returns `style::style_structs::Font.font_size.size()`
    fn size(&self) -> Au;
    /// Returns `style::style_structs::Font.hash`
    fn hash(&self) -> u64;
    /// Returns `style::style_structs::Font.font_weight`
    fn font_weight(&self) -> FontWeight;
    /// Returns `style::style_structs::Font.font_stretch`
    fn font_stretch(&self) -> FontStretch;
    /// Returns `style::style_structs::Font.font_variant_caps`
    fn font_variant_caps(&self) -> FontVariantCaps;
    /// Calls `f` with each family_name in `style::style_structs::Font.font_family`
    fn each_font_family<F>(&self, f: F)
    where F: FnMut(&str);
    /// Wether `style::style_structs::Font.font_style` is either Oblique or Italic
    fn is_oblique_or_italic(&self) -> bool;
}

/// A source for a font-face rule
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg(feature = "servo")]
pub enum Source {
    /// A `url()` source
    Url(Option<ServoUrl>),
    /// A `local()` source
    Local(Atom)
}

/// A list of effective sources that we send over through IPC to the font cache.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg(feature = "servo")]
pub struct EffectiveSources(pub Vec<Source>);

#[cfg(feature = "servo")]
impl Iterator for EffectiveSources {
    type Item = Source;
    fn next(&mut self) -> Option<Source> {
        self.0.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len(), Some(self.0.len()))
    }
}
