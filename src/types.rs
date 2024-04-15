//! Types of all values stored in Exif data as defined in the
//! [list of Exif tags](https://exiv2.org/tags.html).

use fraction::GenericFraction;

/// An 8-bit unsigned integer.
pub type Byte = u8;

/// A 16-bit signed integer.
pub type SShort = i16;

/// A 16-bit unsigned integer.
pub type Short = u16;

/// A 32-bit unsigned integer.
pub type Long = u32;

/// An [`i32`] rational number.
pub type SRational = GenericFraction<i32>;

/// A [`Long`] rational number.
pub type Rational = GenericFraction<Long>;

/// A 32-bit floating-point number.
pub type Float = f32;

/// A 64-bit floating-point number.
pub type Double = f64;

/// An ASCII string.
pub type Ascii = String;

/// An [`Ascii`] comment string.
pub type Comment = Ascii;

/// An undefined and arbitrary sequence of [`Byte`]s.
pub type Undefined = Vec<Byte>;
