//! Types of all values stored in EXIF data as defined in the
//! [EXIF 3.0 Standard](https://www.cipa.jp/std/documents/download_e.html?DC-008-Translation-2023-E).

use fraction::GenericFraction;

/// An 8-bit unsigned integer.
pub type Byte = u8;

/// A 16-bit (2-byte) signed integer (2's complement notation).
pub type SShort = i16;

/// A 16-bit (2-byte) unsigned integer.
pub type Short = u16;

/// A 32-bit (4-byte) signed integer (2's complement notation).
pub type SLong = i32;

/// A 32-bit (4-byte) unsigned integer.
pub type Long = u32;

/// Two [`SLong`]s.
///
/// The first [`SLong`] is the numerator and the second [`SLong`] is the denominator.
pub type SRational = GenericFraction<SLong>;

/// Two [`Long`]s.
///
/// The first [`Long`] is the numerator and the second [`Long`] expresses the denominator.
pub type Rational = GenericFraction<Long>;

/// A 32-bit floating-point number.
pub type Float = f32;

/// A 64-bit floating-point number.
pub type Double = f64;

/// An 8-bit byte containing one 7-bit ASCII code.
///
/// The final byte is terminated with `NULL`. The ASCII count shall include `NULL`.
pub type Ascii = String;

/// An 8-bit byte representing a string according to [`UTF-8`](https://en.wikipedia.org/wiki/UTF-8).
///
/// - The final byte is terminated with `NULL`.
/// - *BOM (Byte Order Mark)* shall not be used.
/// - The UTF-8 count shall include `NULL`.
///
/// This is defined independently by this standard, rather than in `TIFF6.0`.
pub type UTF8 = String;

/// An [`Ascii`] comment string.
pub type Comment = Ascii;

/// An 8-bit [`Byte`] that may take any value depending on the field definition.
pub type Undefined = Vec<Byte>;
