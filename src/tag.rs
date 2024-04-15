//! [Exif tags](https://exiv2.org/tags.html) as defined in the
//! [Exif 2.3 standard](https://www.cipa.jp/std/documents/e/DC-008-2012_E.pdf).

mod image;
pub use image::Image;

/// [Exif tags](https://exiv2.org/tags.html) as defined in the
/// [Exif 2.3 standard](https://www.cipa.jp/std/documents/e/DC-008-2012_E.pdf).
#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    /// Exif Image IFD0 tag.
    Image(Image),

    /// Exif Photo IFD0 tag.
    Photo,

    /// Exif Interoperability IFD0 tag.
    Iop,

    /// Exif GPS Info IFD0 tag.
    GPSInfo,

    /// Exif MPF Info IFD0 tag.
    MpfInfo,

    /// Exif Thumbnail IFD1 tag.
    Thumbnail,
}
