//! Exif Image IFD0 tags.

use crate::types::{
    Ascii, Byte, Double, Float, Long, Rational, SRational, SShort, Short, Undefined,
};

/// Exif Image IFD0 tags.
#[derive(Clone, Debug, PartialEq)]
#[repr(u16)]
pub enum Image {
    /// The name and version of the software used to post-process the picture.
    ProcessingSoftware(Ascii) = 0x000B,

    /// A general indication of the kind of data contained in this subfile.
    NewSubfileType(Long) = 0x00FE,

    /// A general indication of the kind of data contained in this subfile.
    ///
    /// This field is **deprecated**.
    /// The [`Image::NewSubfileType`] field should be used instead.
    #[deprecated(
        since = "0.1.0",
        note = "Use [`Image::NewSubfileType`](#variant.NewSubfileType) instead"
    )]
    SubfileType(Short) = 0x00FF,

    /// The number of columns of image data, equal to the number of pixels per row.
    ///
    /// - In JPEG compressed data a JPEG marker is used instead of this tag.
    ImageWidth(Long) = 0x0100,

    /// The number of rows of image data.
    ///
    /// - In JPEG compressed data a JPEG marker is used instead of this tag.
    ImageLength(Long) = 0x0101,

    /// The number of bits per image component.
    ///
    /// In this standard each component of the image is `8` bits,
    /// so the value for this tag is `8`.
    ///
    /// See also [`Image::SamplesPerPixel`].
    ///
    /// - In JPEG compressed data a JPEG marker is used instead of this tag.
    BitsPerSample(Short) = 0x0102,

    /// The compression scheme used for the image data.
    ///
    /// - When a primary image is JPEG compressed, this designation is not necessary
    ///   and is omitted.
    /// - When thumbnails use JPEG compression, this tag value is set to `6`.
    Compression(Short) = 0x0103,

    /// The pixel composition.
    ///
    /// - In JPEG compressed data a JPEG marker is used instead of this tag.
    PhotometricInterpretation(Short) = 0x0106,

    /// For black and white TIFF files that represent shades of gray,
    /// the technique used to convert from gray to black and white pixels.
    Thresholding(Short) = 0x0107,

    /// The width of the dithering or halftoning matrix used to create
    /// a dithered or halftoned bilevel file.
    CellWidth(Short) = 0x0108,

    /// The length of the dithering or halftoning matrix used to create
    /// a dithered or halftoned bilevel file.
    CellLength(Short) = 0x0109,

    /// The logical order of bits within a byte.
    FillOrder(Short) = 0x010A,

    /// The name of the document from which this image was scanned.
    DocumentName(Ascii) = 0x010D,

    /// A character string giving the title of the image.
    ///
    /// It may be a comment such as `"1988 company picnic"` or the like.
    /// Two-bytes character codes cannot be used. When a `2`-bytes code is necessary,
    /// the Exif Private tag [`Photo::UserComment`] is to be used.
    ImageDescription(Ascii) = 0x010E,

    /// The manufacturer of the recording equipment.
    ///
    /// This is the manufacturer of the DSC, scanner, video digitizer or
    /// other equipment that generated the image. When the field is left blank,
    /// it is treated as unknown.
    Make(Ascii) = 0x010F,

    /// The model name or model number of the equipment.
    ///
    /// This is the model name or number of the DSC, scanner, video digitizer
    /// or other equipment that generated the image. When the field is left blank,
    /// it is treated as unknown.
    Model(Ascii) = 0x0110,

    /// For each strip, the byte offset of that strip.
    ///
    /// It is recommended that this be selected so the number of strip bytes
    /// does not exceed `64KB`.
    ///
    /// - With JPEG compressed data this designation is not needed and is omitted.
    ///
    /// See also [`Image::RowsPerStrip`] and [`Image::StripByteCounts`].
    StripOffsets(Long) = 0x0111,

    /// The image orientation viewed in terms of rows and columns.
    Orientation(Short) = 0x0112,

    /// The number of components per pixel.
    ///
    /// Since this standard applies to RGB and YCbCr images,
    /// the value set for this tag is `3`.
    ///
    /// - In JPEG compressed data a JPEG marker is used instead of this tag.
    SamplesPerPixel(Short) = 0x0115,

    /// The number of rows per strip.
    ///
    /// This is the number of rows in the image of one strip when an image
    /// is divided into strips.
    ///
    /// - With JPEG compressed data this designation is not needed and is omitted.
    ///
    /// See also [`Image::StripOffsets`] and [`Image::StripByteCounts`].
    RowsPerStrip(Long) = 0x0116,

    /// The total number of bytes in each strip.
    ///
    /// - With JPEG compressed data this designation is not needed and is omitted.
    StripByteCounts(Long) = 0x0117,

    /// The number of pixels per [`Image::ResolutionUnit`] in
    /// the [`Image::ImageWidth`] direction.
    ///
    /// When the image resolution is unknown, `72 [dpi]` is designated.
    XResolution(Rational) = 0x011A,

    /// The number of pixels per [`Image::ResolutionUnit`] in
    /// the [`Image::ImageLength`] direction.
    ///
    /// The same value as [`Image::XResolution`] is designated.
    YResolution(Rational) = 0x011B,

    /// Indicates whether pixel components are recorded in a chunky or planar format.
    ///
    /// - In JPEG compressed files a JPEG marker is used instead of this tag.
    ///
    /// If this field does not exist, the TIFF default of `1` (chunky) is assumed.
    PlanarConfiguration(Short) = 0x011C,

    /// The name of the page from which this image was scanned.
    PageName(Ascii) = 0x011D,

    /// X position of the image.
    ///
    /// The X offset in [`Image::ResolutionUnit`]s of the left side of the image,
    /// with respect to the left side of the page.
    XPosition(Rational) = 0x011E,

    /// Y position of the image.
    ///
    /// The Y offset in [`Image::ResolutionUnit`]s of the top of the image,
    /// with respect to the top of the page.
    ///
    /// In the TIFF coordinate scheme, the positive Y direction is down,
    /// so that [`Image::YPosition`] is always positive.
    YPosition(Rational) = 0x011F,

    /// The precision of the information contained in the [`Image::GrayResponseCurve`].
    GrayResponseUnit(Short) = 0x0122,

    /// For grayscale data, the optical density of each possible pixel value.
    GrayResponseCurve(Short) = 0x0123,

    /// `T.4`-encoding options.
    T4Options(Long) = 0x0124,

    /// `T.6`-encoding options.
    T6Options(Long) = 0x0125,

    /// The unit for measuring [`Image::XResolution`] and [`Image::YResolution`].
    ///
    /// The same unit is used for both [`Image::XResolution`] and [`Image::YResolution`].
    ///
    /// If the image resolution is unknown, `2` (inches) is designated.
    ResolutionUnit(Short) = 0x0128,

    /// The page number of the page from which this image was scanned.
    PageNumber(Short) = 0x0129,

    /// A transfer function for the image, described in tabular style.
    ///
    /// Normally this tag is not necessary, since color space is specified in
    /// the color space information tag ([`Photo::ColorSpace`]).
    TransferFunction(Short) = 0x012D,

    /// This tag records the name and version of the software or firmware of
    /// the camera or image input device used to generate the image.
    ///
    /// The detailed format is not specified, but it is recommended that
    /// the example shown below be followed.
    ///
    /// When the field is left blank, it is treated as unknown.
    Software(Ascii) = 0x0131,

    /// The date and time of image creation.
    ///
    /// In Exif standard, it is the date and time the file was changed.
    DateTime(Ascii) = 0x0132,

    /// This tag records the name of the camera owner, photographer or image creator.
    ///
    /// The detailed format is not specified, but it is recommended that the information
    /// be written as in the example below for ease of Interoperability.
    ///
    /// When the field is left blank, it is treated as unknown.
    ///
    /// # Examples
    ///
    /// `"Camera owner, John Smith; Photographer, Michael Brown; Image creator, Ken James"`
    Artist(Ascii) = 0x013B,

    /// This tag records information about the host computer used to generate the image.
    HostComputer(Ascii) = 0x013C,

    /// A predictor is a mathematical operator that is applied to the image data before
    /// an encoding scheme is applied.
    Predictor(Short) = 0x013D,

    /// The chromaticity of the white point of the image.
    ///
    /// Normally this tag is not necessary, since color space is specified in
    /// the colorspace information tag ([`Photo::ColorSpace`]).
    WhitePoint(Rational) = 0x013E,

    /// The chromaticity of the three primary colors of the image.
    ///
    /// Normally this tag is not necessary, since colorspace is specified in
    /// the colorspace information tag ([`Photo::ColorSpace`]).
    PrimaryChromaticities(Rational) = 0x013F,

    /// A color map for palette color images.
    ///
    /// This field defines a Red-Green-Blue color map (often called a lookup table)
    /// for palette-color images.
    ///
    /// In a palette-color image, a pixel value is used to index into an RGB lookup table.
    ColorMap(Short) = 0x0140,

    /// The purpose of the [`Image::HalftoneHints`] field is to convey to the halftone
    /// function the range of gray levels within a colorimetrically-specified image
    /// that should retain tonal detail.
    HalftoneHints(Short) = 0x0141,

    /// The tile width in pixels.
    ///
    /// This is the number of columns in each tile.
    TileWidth(Long) = 0x0142,

    /// The tile length (height) in pixels.
    ///
    /// This is the number of rows in each tile.
    TileLength(Long) = 0x0143,

    /// For each tile, the byte offset of that tile, as compressed and stored on disk.
    ///
    /// The offset is specified with respect to the beginning of the TIFF file.
    /// Note that this implies that each tile has a location independent of
    /// the locations of other tiles.
    TileOffsets(Short) = 0x0144,

    /// For each tile, the number of (compressed) bytes in that tile.
    ///
    /// See [`Image::TileOffsets`] for a description of how the byte counts are ordered.
    TileByteCounts(Long) = 0x0145,

    /// Defined by Adobe Corporation to enable TIFF Trees within a TIFF file.
    SubIFDs(Long) = 0x014A,

    /// The set of inks used in a separated ([`Image::PhotometricInterpretation`]=`5`) image.
    InkSet(Short) = 0x014C,

    /// The name of each ink used in a separated ([`Image::PhotometricInterpretation`]=`5`) image.
    InkNames(Ascii) = 0x014D,

    /// The number of inks.
    ///
    /// Usually equal to [`Image::SamplesPerPixel`], unless there are extra samples.
    NumberOfInks(Short) = 0x014E,

    /// The component values that correspond to a `0%` dot and `100%` dot.
    DotRange(Byte) = 0x0150,

    /// A description of the printing environment for which this separation is intended.
    TargetPrinter(Ascii) = 0x0151,

    /// Specifies that each pixel has `m` extra components whose interpretation
    /// is defined by one of the values listed below.
    ExtraSamples(Short) = 0x0152,

    /// This field specifies how to interpret each data sample in a pixel.
    SampleFormat(Short) = 0x0153,

    /// This field specifies the minimum sample value.
    SMinSampleValue(Short) = 0x0154,

    /// This field specifies the maximum sample value.
    SMaxSampleValue(Short) = 0x0155,

    /// Expands the range of the [`Image::TransferFunction`].
    TransferRange(Short) = 0x0156,

    /// A TIFF [`Image::ClipPath`] is intended to mirror the essentials of
    /// `PostScript`'s path creation functionality.
    ClipPath(Byte) = 0x0157,

    /// The number of units that span the width of the image,
    /// in terms of integer [`Image::ClipPath`] coordinates.
    XClipPathUnits(SShort) = 0x0158,

    /// The number of units that span the height of the image,
    /// in terms of integer [`Image::ClipPath`] coordinates.
    YClipPathUnits(SShort) = 0x0159,

    /// Indexed images are images where the 'pixels' do not represent color values,
    /// but rather an index (usually `8`-bit) into a separate color table,
    /// the [`Image::ColorMap`].
    Indexed(Short) = 0x015A,

    /// This optional tag may be used to encode the JPEG quantization and
    /// Huffman tables for subsequent use by the JPEG decompression process.
    JPEGTables(Undefined) = 0x015B,

    /// [`Image::OPIProxy`] gives information concerning whether this image is
    /// a low-resolution proxy of a high-resolution image (Adobe OPI).
    OPIProxy(Short) = 0x015F,

    /// This field indicates the process used to produce the compressed data.
    JPEGProc(Long) = 0x0200,

    /// The offset to the start byte (SOI) of JPEG compressed thumbnail data.
    ///
    /// This is not used for primary image JPEG data.
    JPEGInterchangeFormat(Long) = 0x0201,

    /// The number of bytes of JPEG compressed thumbnail data.
    ///
    /// This is not used for primary image JPEG data.
    ///
    /// JPEG thumbnails are not divided but are recorded as a continuous
    /// JPEG bitstream from SOI to EOI. Appn and COM markers should not be recorded.
    /// Compressed thumbnails must be recorded in no more than `64KB`, including all
    /// other data to be recorded in APP1.
    JPEGInterchangeFormatLength(Long) = 0x0202,

    /// This Field indicates the length of the restart interval used in
    /// the compressed image data.
    JPEGRestartInterval(Short) = 0x0203,

    /// This Field points to a list of lossless predictor-selection values,
    /// one per component.
    JPEGLosslessPredictors(Short) = 0x0205,

    /// This Field points to a list of point transform values, one per component.
    JPEGPointTransforms(Short) = 0x0206,

    /// This Field points to a list of offsets to the quantization tables,
    /// one per component.
    JPEGQTables(Long) = 0x0207,

    /// This Field points to a list of offsets to the DC Huffman tables or
    /// the lossless Huffman tables, one per component.
    JPEGDCTables(Long) = 0x0208,

    /// This Field points to a list of offsets to the Huffman AC tables,
    /// one per component.
    JPEGACTables(Long) = 0x0209,

    /// The matrix coefficients for transformation from RGB to YCbCr image data.
    ///
    /// No default is given in TIFF; but here the value given in
    /// [Appendix E, "Color Space Guidelines"](https://www.cipa.jp/std/documents/e/DC-008-2012_E.pdf),
    /// is used as the default.
    ///
    /// The color space is declared in a color space information tag, with
    /// the default being the value that gives the optimal image characteristics
    /// Interoperability this condition.
    YCbCrCoefficients(Rational) = 0x0211,

    /// The sampling ratio of chrominance components in relation to
    /// the luminance component.
    ///
    /// - In JPEG compressed data a JPEG marker is used instead of this tag.
    YCbCrSubSampling(Short) = 0x0212,

    /// The position of chrominance components in relation to the luminance component.
    ///
    /// This field is designated only for JPEG compressed data or uncompressed YCbCr data.
    /// The TIFF default is `1` (centered); but when Y:Cb:Cr = `4:2:2` it is recommended in
    /// this standard that `2` (co-sited) be used to record data, in order to improve
    /// the image quality when viewed on TV systems.
    ///
    /// When this field does not exist, the reader shall assume the TIFF default.
    ///
    /// In the case of Y:Cb:Cr = `4:2:0`, the TIFF default (centered) is recommended.
    ///
    /// If the reader does not have the capability of supporting both kinds of
    /// [`Image::YCbCrPositioning`], it shall follow the TIFF default regardless of
    /// the value in this field.
    ///
    /// It is preferable that readers be able to support both centered and co-sited positioning.
    YCbCrPositioning(Short) = 0x0213,

    /// The reference black point value and reference white point value.
    ///
    /// No defaults are given in TIFF, but the values below are given as defaults here.
    ///
    /// The color space is declared in a color space information tag, with the default
    /// being the value that gives the optimal image characteristics Interoperability
    /// these conditions.
    ReferenceBlackWhite(Rational) = 0x0214,

    /// XMP Metadata (Adobe technote `9-14-02`).
    XMLPacket(Byte) = 0x02BC,

    /// Rating tag used by Windows.
    Rating(Short) = 0x4746,

    /// Rating tag used by Windows, value in percent.
    RatingPercent(Short) = 0x4749,

    /// Sony vignetting correction parameters.
    VignettingCorrParams(SShort) = 0x7032,

    /// Sony chromatic aberration correction parameters.
    ChromaticAberrationCorrParams(SShort) = 0x7035,

    /// Sony distortion correction parameters.
    DistortionCorrParams(SShort) = 0x7037,

    /// [`Image::ImageID`] is the full pathname of the original, high-resolution image,
    /// or any other identifying string that uniquely identifies the original image (Adobe OPI).
    ImageID(Ascii) = 0x800D,

    /// Contains two values representing the minimum rows and columns to define
    /// the repeating patterns of the color filter array.
    CFARepeatPatternDim(Short) = 0x828D,

    /// Indicates the color filter array (CFA) geometric pattern of the image sensor
    /// when a one-chip color area sensor is used.
    ///
    /// It does not apply to all sensing methods.
    CFAPattern(Byte) = 0x828E,

    /// Contains a value of the battery level as a fraction ~~or string~~.
    BatteryLevel(Rational) = 0x828F,

    /// Copyright information.
    ///
    /// In this standard the tag is used to indicate both the photographer and
    /// editor copyrights. It is the copyright notice of the person or organization
    /// claiming rights to the image. The Interoperability copyright statement including
    /// date and rights should be written in this field.
    ///
    /// In this standard the field records both the photographer and editor copyrights,
    /// with each recorded in a separate part of the statement. When there is a clear
    /// distinction between the photographer and editor copyrights, these are to be
    /// written in the order of photographer followed by editor copyright, separated by
    /// `NULL` (in this case since the statement also ends with a `NULL`, there are two
    /// `NULL` codes). When only the photographer copyright is given, it is terminated by
    /// one `NULL` code. When only the editor copyright is given, the photographer
    /// copyright part consists of one space followed by a terminating `NULL` code,
    /// then the editor copyright is given. When the field is left blank, it is treated
    /// as unknown.
    ///
    /// # Examples
    ///
    /// `"Copyright, John Smith, 19xx. All rights reserved."`
    Copyright(Ascii) = 0x8298,

    /// Exposure time, given in seconds.
    ExposureTime(Rational) = 0x829A,

    /// The F number.
    FNumber(Rational) = 0x829D,

    /// Contains an IPTC/NAA record.
    IPTCNAA(Long) = 0x83BB,

    /// Contains information embedded by the Adobe Photoshop application.
    ImageResources(Byte) = 0x8649,

    /// A pointer to the Exif IFD.
    ///
    /// Interoperability, Exif IFD has the same structure as that of the IFD
    /// specified in TIFF. Ordinarily, however, it does not contain image data
    /// as in the case of TIFF.
    ExifTag(Long) = 0x8769,

    /// Contains an InterColor Consortium (ICC) format color space
    /// characterization/profile.
    InterColorProfile(Undefined) = 0x8773,

    /// The class of the program used by the camera to set exposure when the picture
    /// is taken.
    ExposureProgram(Short) = 0x8822,

    /// Indicates the spectral sensitivity of each channel of the camera used.
    SpectralSensitivity(Ascii) = 0x8824,

    /// A pointer to the GPS Info IFD.
    ///
    /// The Interoperability structure of the GPS Info IFD, like that of Exif IFD,
    /// has no image data.
    GPSTag(Long) = 0x8825,

    /// Indicates the ISO Speed and ISO Latitude of the camera or input device as
    /// specified in `ISO 12232`.
    ISOSpeedRatings(Short) = 0x8827,

    /// Indicates the Opto-Electric Conversion Function (OECF) specified in `ISO 14524`.
    OECF(Undefined) = 0x8828,

    /// Indicates the field number of multifield images.
    Interlace(Short) = 0x8829,

    /// This optional tag encodes the time zone of the camera clock (relative to
    /// Greenwich Mean Time) used to create the [`Image::DateTimeOriginal`]
    /// tag-value when the picture was taken.
    ///
    /// It may also contain the time zone offset of the clock used to create the
    /// [`Image::DateTime`] tag-value when the image was modified.
    TimeZoneOffset(SShort) = 0x882A,

    /// Number of seconds image capture was delayed from button press.
    SelfTimerMode(Short) = 0x882B,

    /// The date and time when the original image data was generated.
    DateTimeOriginal(Ascii) = 0x9003,

    /// Specific to compressed data; states the compressed bits per pixel.
    CompressedBitsPerPixel(Rational) = 0x9102,

    /// Shutter speed.
    ShutterSpeedValue(SRational) = 0x9201,

    /// The lens aperture.
    ApertureValue(Rational) = 0x9202,

    /// The value of brightness.
    BrightnessValue(SRational) = 0x9203,

    /// The exposure bias.
    ExposureBiasValue(SRational) = 0x9204,

    /// The smallest F number of the lens.
    MaxApertureValue(Rational) = 0x9205,

    /// The distance to the subject, given in meters.
    SubjectDistance(SRational) = 0x9206,

    /// The metering mode.
    MeteringMode(Short) = 0x9207,

    /// The kind of light source.
    LightSource(Short) = 0x9208,

    /// Indicates the status of flash when the image was shot.
    Flash(Short) = 0x9209,

    /// The actual focal length of the lens, in `mm`.
    FocalLength(Rational) = 0x920A,

    /// Amount of flash energy (BCPS).
    FlashEnergy(Rational) = 0x920B,

    /// SFR of the camera.
    SpatialFrequencyResponse(Undefined) = 0x920C,

    /// Noise measurement values.
    Noise(Undefined) = 0x920D,

    /// Number of pixels per [`Image::FocalPlaneResolutionUnit`] in
    /// [`Image::ImageWidth`] direction for main image.
    FocalPlaneXResolution(Rational) = 0x920E,

    /// Number of pixels per [`Image::FocalPlaneResolutionUnit`] in
    /// [`Image::ImageLength`] direction for main image.
    FocalPlaneYResolution(Rational) = 0x920F,

    /// Unit of measurement for [`Image::FocalPlaneXResolution`] and
    /// [`Image::FocalPlaneYResolution`].
    FocalPlaneResolutionUnit(Short) = 0x9210,

    /// Number assigned to an image, e.g., in a chained image burst.
    ImageNumber(Long) = 0x9211,

    /// Security classification assigned to the image.
    SecurityClassification(Ascii) = 0x9212,

    /// Record of what has been done to the image.
    ImageHistory(Ascii) = 0x9213,

    /// Indicates the location and area of the main subject in the overall scene.
    SubjectLocation(Short) = 0x9214,

    /// Encodes the camera exposure index setting when image was captured.
    ExposureIndex(Rational) = 0x9215,

    /// Contains four ASCII characters representing the TIFF/EP standard version
    /// of a TIFF/EP file.
    ///
    /// # Examples
    ///
    /// `'1'`, `'0'`, `'0'`, `'0'`
    TIFFEPStandardID(Byte) = 0x9216,

    /// Type of image sensor.
    SensingMethod(Short) = 0x9217,

    /// Title tag used by Windows, encoded in UCS2.
    XPTitle(Byte) = 0x9C9B,

    /// Comment tag used by Windows, encoded in UCS2.
    XPComment(Byte) = 0x9C9C,

    /// Author tag used by Windows, encoded in UCS2.
    XPAuthor(Byte) = 0x9C9D,

    /// Keywords tag used by Windows, encoded in UCS2.
    XPKeywords(Byte) = 0x9C9E,

    /// Subject tag used by Windows, encoded in UCS2.
    XPSubject(Byte) = 0x9C9F,

    /// Print Image Matching, description needed.
    PrintImageMatching(Undefined) = 0xC4A5,

    /// This tag encodes the DNG four-tier version number.
    ///
    /// For files compliant with version `1.1.0.0` of the DNG specification,
    /// this tag should contain the bytes: `1`, `1`, `0`, `0`.
    DNGVersion(Byte) = 0xC612,

    /// This tag specifies the oldest version of the Digital Negative specification
    /// for which a file is compatible.
    ///
    /// Readers should not attempt to read a file if this tag specifies a version
    /// number that is higher than the version number of the specification the reader
    /// was based on. In addition to checking the version tags, readers should,
    /// for all tags, check the types, counts, and values, to verify it is able
    /// to correctly read the file.
    DNGBackwardVersion(Byte) = 0xC613,

    /// Defines a unique, non-localized name for the camera model that created
    /// the image in the raw file.
    ///
    /// This name should include the manufacturer's name to avoid conflicts, and
    /// should not be localized, even if the camera name itself is localized for
    /// different markets (see [`Image::LocalizedCameraModel`]). This string may
    /// be used by reader software to index into per-model preferences and
    /// replacement profiles.
    UniqueCameraModel(Ascii) = 0xC614,

    /// Similar to the [`Image::UniqueCameraModel`] field, except the name can be
    /// localized for different markets to match the localization of the camera name.
    LocalizedCameraModel(Byte) = 0xC615,

    /// Provides a mapping between the values in the [`Image::CFAPattern`] tag and
    /// the plane numbers in `LinearRaw` space.
    ///
    /// This is a required tag for non-RGB CFA images.
    CFAPlaneColor(Byte) = 0xC616,

    /// Describes the spatial layout of the CFA.
    CFALayout(Short) = 0xC617,

    /// Describes a lookup table that maps stored values into linear values.
    ///
    /// This tag is typically used to increase compression ratios by storing
    /// the raw data in a non-linear, more visually uniform space with fewer
    /// total encoding levels. If [`Image::SamplesPerPixel`] is not equal to one,
    /// this single table applies to all the samples for each pixel.
    LinearizationTable(Short) = 0xC618,

    /// Specifies repeat pattern size for the [`Image::BlackLevel`] tag.
    BlackLevelRepeatDim(Short) = 0xC619,

    /// Specifies the zero light (a.k.a. thermal black or black current) encoding level,
    /// as a repeating pattern.
    ///
    /// The origin of this pattern is the top-left corner of the [`Image::ActiveArea`]
    /// rectangle. The values are stored in row-column-sample scan order.
    BlackLevel(Rational) = 0xC61A,

    /// If the zero light encoding level is a function of the image column,
    /// [`Image::BlackLevelDeltaH`] specifies the difference between the zero light
    /// encoding level for each column and the baseline zero light encoding level.
    ///
    /// If [`Image::SamplesPerPixel`] is not equal to one, this single table applies
    /// to all the samples for each pixel.
    BlackLevelDeltaH(SRational) = 0xC61B,

    /// If the zero light encoding level is a function of the image row, this tag
    /// specifies the difference between the zero light encoding level for each row
    /// and the baseline zero light encoding level.
    ///
    /// If [`Image::SamplesPerPixel`] is not equal to one, this single table applies
    /// to all the samples for each pixel.
    BlackLevelDeltaV(SRational) = 0xC61C,

    /// This tag specifies the fully saturated encoding level for the raw sample values.
    ///
    /// Saturation is caused either by the sensor itself becoming highly non-linear in response,
    /// or by the camera's analog to digital converter clipping.
    WhiteLevel(Long) = 0xC61D,

    /// [`Image::DefaultScale`] is required for cameras with non-square pixels.
    ///
    /// It specifies the default scale factors for each direction to convert the image
    /// to square pixels. Typically these factors are selected to approximately preserve
    /// total pixel count.
    ///
    /// For CFA images that use CFALayout equal to `2`, `3`, `4`, or `5`, such as
    /// the Fujifilm SuperCCD, these two values should usually differ by a factor of `2.0`.
    DefaultScale(Rational) = 0xC61E,

    /// Raw images often store extra pixels around the edges of the final image.
    ///
    /// These extra pixels help prevent interpolation artifacts near the edges of
    /// the final image. [`Image::DefaultCropOrigin`] specifies the origin of the
    /// final image area, in raw image coordinates (i.e., before the
    /// [`Image::DefaultScale`] has been applied), relative to the top-left corner
    /// of the [`Image::ActiveArea`] rectangle.
    DefaultCropOrigin(Long) = 0xC61F,

    /// Raw images often store extra pixels around the edges of the final image.
    ///
    /// These extra pixels help prevent interpolation artifacts near the edges of
    /// the final image. [`Image::DefaultCropSize`] specifies the size of the final
    /// image area, in raw image coordinates (i.e., before the
    /// [`Image::DefaultScale`] has been applied).
    DefaultCropSize(Long) = 0xC620,

    /// [`Image::ColorMatrix1`] defines a transformation matrix that converts
    /// XYZ values to reference camera native color space values, under the first
    /// calibration illuminant.
    ///
    /// The matrix values are stored in row scan order. The [`Image::ColorMatrix1`]
    /// tag is required for all non-monochrome DNG files.
    ColorMatrix1(SRational) = 0xC621,

    /// [`Image::ColorMatrix2`] defines a transformation matrix that converts
    /// XYZ values to reference camera native color space values, under the second
    /// calibration illuminant.
    ///
    /// The matrix values are stored in row scan order.
    ColorMatrix2(SRational) = 0xC622,

    /// [`Image::CameraCalibration1`] defines a calibration matrix that transforms
    /// reference camera native space values to individual camera native space values
    /// under the first calibration illuminant.
    ///
    /// The matrix is stored in row scan order. This matrix is stored separately from
    /// the matrix specified by the [`Image::ColorMatrix1`] tag to allow raw converters
    /// to swap in replacement color matrices based on [`Image::UniqueCameraModel`] tag,
    /// while still taking advantage of any per-individual camera calibration performed
    /// by the camera manufacturer.
    CameraCalibration1(SRational) = 0xC623,

    /// [`Image::CameraCalibration2`] defines a calibration matrix that transforms
    /// reference camera native space values to individual camera native space values
    /// under the second calibration illuminant.
    ///
    /// The matrix is stored in row scan order. This matrix is stored separately from
    /// the matrix specified by the [`Image::ColorMatrix2`] tag to allow raw converters
    /// to swap in replacement color matrices based on [`Image::UniqueCameraModel`] tag,
    /// while still taking advantage of any per-individual camera calibration performed
    /// by the camera manufacturer.
    CameraCalibration2(SRational) = 0xC624,

    /// [`Image::ReductionMatrix1`] defines a dimensionality reduction matrix for use
    /// as the first stage in converting color camera native space values to XYZ values,
    /// under the first calibration illuminant.
    ///
    /// This tag may only be used if `ColorPlanes` is greater than `3`.
    /// The matrix is stored in row scan order.
    ReductionMatrix1(SRational) = 0xC625,

    /// [`Image::ReductionMatrix2`] defines a dimensionality reduction matrix for use
    /// as the first stage in converting color camera native space values to XYZ values,
    /// under the second calibration illuminant.
    ///
    /// This tag may only be used if `ColorPlanes` is greater than `3`.
    /// The matrix is stored in row scan order.
    ReductionMatrix2(SRational) = 0xC626,

    /// Normally the stored raw values are not white balanced, since any digital
    /// white balancing will reduce the dynamic range of the final image if the user
    /// decides to later adjust the white balance; however, if camera hardware is
    /// capable of white balancing the color channels before the signal is digitized,
    /// it can improve the dynamic range of the final image.
    ///
    /// [`Image::AnalogBalance`] defines the gain, either analog (recommended) or
    /// digital (not recommended) that has been applied the stored raw values.
    AnalogBalance(Rational) = 0xC627,

    /// Specifies the selected white balance at time of capture, encoded as the
    /// coordinates of a perfectly neutral color in linear reference space values.
    ///
    /// The inclusion of this tag precludes the inclusion of the
    /// [`Image::AsShotWhiteXY`] tag.
    AsShotNeutral(Short) = 0xC628,

    /// Specifies the selected white balance at time of capture, encoded as
    /// `x-y` chromaticity coordinates.
    ///
    /// The inclusion of this tag precludes the inclusion of the
    /// [`Image::AsShotNeutral`] tag.
    AsShotWhiteXY(Rational) = 0xC629,

    /// Camera models vary in the trade-off they make between highlight headroom
    /// and shadow noise.
    ///
    /// Some leave a significant amount of highlight headroom during a normal exposure.
    /// This allows significant negative exposure compensation to be applied during
    /// raw conversion, but also means normal exposures will contain more shadow noise.
    /// Other models leave less headroom during normal exposures. This allows for less
    /// negative exposure compensation, but results in lower shadow noise for normal
    /// exposures. Because of these differences, a raw converter needs to vary the zero
    /// point of its exposure compensation control from model to model.
    /// [`Image::BaselineExposure`] specifies by how much (in EV units) to move the zero
    /// point. Positive values result in brighter default results, while negative values
    /// result in darker default results.
    BaselineExposure(SRational) = 0xC62A,

    /// Specifies the relative noise level of the camera model at a baseline ISO value
    /// of `100`, compared to a reference camera model.
    ///
    /// Since noise levels tend to vary approximately with the square root of the
    /// ISO value, a raw converter can use this value, combined with the current ISO,
    /// to estimate the relative noise level of the current image.
    BaselineNoise(Rational) = 0xC62B,

    /// Specifies the relative amount of sharpening required for this camera model,
    /// compared to a reference camera model.
    ///
    /// Camera models vary in the strengths of their anti-aliasing filters.
    /// Cameras with weak or no filters require less sharpening than cameras with
    /// strong anti-aliasing filters.
    BaselineSharpness(Rational) = 0xC62C,

    /// Only applies to CFA images using a Bayer pattern filter array.
    ///
    /// This tag specifies, in arbitrary units, how closely the values of the
    /// green pixels in the blue/green rows track the values of the green pixels
    /// in the red/green rows. A value of zero means the two kinds of green pixels
    /// track closely, while a non-zero value means they sometimes diverge.
    /// The useful range for this tag is from `0` (no divergence) to about `5000`
    /// (quite large divergence).
    BayerGreenSplit(Long) = 0xC62D,

    /// Some sensors have an unpredictable non-linearity in their response as they
    /// near the upper limit of their encoding range.
    ///
    /// This non-linearity results in color shifts in the highlight areas of
    /// the resulting image unless the raw converter compensates for this effect.
    /// [`Image::LinearResponseLimit`] specifies the fraction of the encoding range
    /// above which the response may become significantly non-linear.
    LinearResponseLimit(Rational) = 0xC62E,

    /// [`Image::CameraSerialNumber`] contains the serial number of the camera or
    /// camera body that captured the image.
    CameraSerialNumber(Ascii) = 0xC62F,

    /// Contains information about the lens that captured the image.
    ///
    /// If the minimum f-stops are unknown, they should be encoded as `0/0`.
    LensInfo(Rational) = 0xC630,

    /// [`Image::ChromaBlurRadius`] provides a hint to the DNG reader about how
    /// much chroma blur should be applied to the image.
    ///
    /// If this tag is omitted, the reader will use its default amount of
    /// chroma blurring. Normally this tag is only included for non-CFA images,
    /// since the amount of chroma blur required for mosaic images is highly dependent
    /// on the de-mosaic algorithm, in which case the DNG reader's default value is
    /// likely optimized for its particular de-mosaic algorithm.
    ChromaBlurRadius(Rational) = 0xC631,

    /// Provides a hint to the DNG reader about how strong the camera's anti-alias
    /// filter is.
    ///
    /// A value of `0.0` means no anti-alias filter (i.e., the camera is prone to
    /// aliasing artifacts with some subjects), while a value of `1.0` means a
    /// strong anti-alias filter (i.e., the camera almost never has aliasing artifacts).
    AntiAliasStrength(Rational) = 0xC632,

    /// This tag is used by Adobe Camera Raw to control the sensitivity of its
    /// `Shadows` slider.
    ShadowScale(SRational) = 0xC633,

    /// Provides a way for camera manufacturers to store private data in the
    /// DNG file for use by their own raw converters, and to have that data
    /// preserved by programs that edit DNG files.
    DNGPrivateData(Byte) = 0xC634,

    /// [`Image::MakerNoteSafety`] lets the DNG reader know whether the
    /// EXIF [`Image::MakerNote`] tag is safe to preserve along with the
    /// rest of the EXIF data.
    ///
    /// File browsers and other image management software processing an image
    /// with a preserved [`Image::MakerNote`] should be aware that any thumbnail
    /// image embedded in the [`Image::MakerNote`] may be stale, and may not
    /// reflect the current state of the full size image.
    MakerNoteSafety(Short) = 0xC635,

    /// The illuminant used for the first set of color calibration tags
    /// ([`Image::ColorMatrix1`], [`Image::CameraCalibration1`], [`Image::ReductionMatrix1`]).
    ///
    /// The legal values for this tag are the same as the legal values for the
    /// [`Image::LightSource`] EXIF tag. If set to `255` (Other), then the IFD
    /// must also include a [`Image::IlluminantData1`] tag to specify the `x-y`
    /// chromaticity or spectral power distribution function for this illuminant.
    CalibrationIlluminant1(Short) = 0xC65A,

    /// The illuminant used for an optional second set of color calibration tags
    /// ([`Image::ColorMatrix2`], [`Image::CameraCalibration2`], [`Image::ReductionMatrix2`]).
    ///
    /// The legal values for this tag are the same as the legal values for the
    /// [`Image::CalibrationIlluminant1`] tag; however, if both are included,
    /// neither is allowed to have a value of `0` (unknown). If set to `255`
    /// (Other), then the IFD must also include a [`Image::IlluminantData2`]
    /// tag to specify the `x-y` chromaticity or spectral power distribution
    /// function for this illuminant.
    CalibrationIlluminant2(Short) = 0xC65B,

    /// For some cameras, the best possible image quality is not achieved by
    /// preserving the total pixel count during conversion.
    ///
    /// For example, Fujifilm SuperCCD images have maximum detail when their
    /// total pixel count is doubled. This tag specifies the amount by which
    /// the values of the [`Image::DefaultScale`] tag need to be multiplied
    /// to achieve the best quality image size.
    BestQualityScale(Rational) = 0xC65C,

    /// This tag contains a 16-byte unique identifier for the raw image data in
    /// the DNG file.
    ///
    /// DNG readers can use this tag to recognize a particular raw image, even if
    /// the file's name or the metadata contained in the file has been changed.
    /// If a DNG writer creates such an identifier, it should do so using an
    /// algorithm that will ensure that it is very unlikely two different images
    /// will end up having the same identifier.
    RawDataUniqueID(Byte) = 0xC65D,

    /// If the DNG file was converted from a non-DNG raw file, then this tag contains
    /// the file name of that original raw file.
    OriginalRawFileName(Byte) = 0xC68B,

    /// If the DNG file was converted from a non-DNG raw file, then this tag contains
    /// the compressed contents of that original raw file.
    ///
    /// The contents of this tag always use the big-endian byte order. The tag
    /// contains a sequence of data blocks. Future versions of the DNG specification
    /// may define additional data blocks, so DNG readers should ignore extra bytes
    /// when parsing this tag. DNG readers should also detect the case where data
    /// blocks are missing from the end of the sequence, and should assume a default
    /// value for all the missing blocks. There are no padding or alignment bytes
    /// between data blocks.
    OriginalRawFileData(Undefined) = 0xC68C,

    /// This rectangle defines the active (non-masked) pixels of the sensor.
    ///
    /// The order of the rectangle coordinates is: `top`, `left`, `bottom`, `right`.
    ActiveArea(Long) = 0xC68D,

    /// This tag contains a list of non-overlapping rectangle coordinates of fully
    /// masked pixels, which can be optionally used by DNG readers to measure the
    /// black encoding level.
    ///
    /// The order of each rectangle's coordinates is: `top`, `left`, `bottom`, `right`.
    ///
    /// If the raw image data has already had its black encoding level subtracted,
    /// then this tag should not be used, since the masked pixels are no longer useful.
    MaskedAreas(Long) = 0xC68E,

    /// This tag contains an ICC profile that, in conjunction with the
    /// [`Image::AsShotPreProfileMatrix`] tag, provides the camera manufacturer
    /// with a way to specify a default color rendering from camera color space
    /// coordinates (linear reference values) into the ICC profile connection space.
    ///
    /// The ICC profile connection space is an output referred colorimetric space,
    /// whereas the other color calibration tags in DNG specify a conversion into
    /// a scene referred colorimetric space. This means that the rendering in this
    /// profile should include any desired tone and gamut mapping needed to convert
    /// between scene referred values and output referred values.
    AsShotICCProfile(Undefined) = 0xC68F,

    /// This tag is used in conjunction with the [`Image::AsShotICCProfile`] tag.
    ///
    /// It specifies a matrix that should be applied to the camera color space
    /// coordinates before processing the values through the ICC profile specified
    /// in the [`Image::AsShotICCProfile`] tag. The matrix is stored in the row scan
    /// order. If `ColorPlanes` is greater than `3`, then this matrix can (but is not
    /// required to) reduce the dimensionality of the color data down to three
    /// components, in which case the [`Image::AsShotICCProfile`] should have three
    /// rather than `ColorPlanes` input components.
    AsShotPreProfileMatrix(SRational) = 0xC690,

    /// This tag is used in conjunction with the [`Image::CurrentPreProfileMatrix`] tag.
    ///
    /// The [`Image::CurrentICCProfile`] and [`Image::CurrentPreProfileMatrix`] tags
    /// have the same purpose and usage as the [`Image::AsShotICCProfile`] and
    /// [`Image::AsShotPreProfileMatrix`] tag pair, except they are for use by raw
    /// file editors rather than camera manufacturers.
    CurrentICCProfile(Undefined) = 0xC691,

    /// This tag is used in conjunction with the [`Image::CurrentICCProfile`] tag.
    ///
    /// The [`Image::CurrentICCProfile`] and [`Image::CurrentPreProfileMatrix`] tags
    /// have the same purpose and usage as the [`Image::AsShotICCProfile`] and
    /// [`Image::AsShotPreProfileMatrix`] tag pair, except they are for use by raw
    /// file editors rather than camera manufacturers.
    CurrentPreProfileMatrix(SRational) = 0xC692,

    /// The DNG color model documents a transform between camera colors and
    /// CIE XYZ values.
    ///
    /// This tag describes the colorimetric reference for the CIE XYZ values.
    ///
    /// - `0` = The XYZ values are scene-referred.
    /// - `1` = The XYZ values are output-referred, using the ICC profile
    ///   perceptual dynamic range.
    ///
    /// This tag allows output-referred data to be stored in DNG files and still
    /// processed correctly by DNG readers.
    ColorimetricReference(Short) = 0xC6BF,

    /// A UTF-8 encoded string associated with the [`Image::CameraCalibration1`] and
    /// [`Image::CameraCalibration2`] tags.
    ///
    /// The [`Image::CameraCalibration1`] and [`Image::CameraCalibration2`] tags
    /// should only be used in the DNG color transform if the string stored in
    /// the [`Image::CameraCalibrationSignature`] tag exactly matches the string
    /// stored in the [`Image::ProfileCalibrationSignature`] tag for the selected
    /// camera profile.
    CameraCalibrationSignature(Byte) = 0xC6F3,

    /// A UTF-8 encoded string associated with the camera profile tags.
    ///
    /// The [`Image::CameraCalibration1`] and [`Image::CameraCalibration2`] tags
    /// should only be used in the DNG color transfer if the string stored in
    /// the [`Image::CameraCalibrationSignature`] tag exactly matches the string
    /// stored in the [`Image::ProfileCalibrationSignature`] tag for the selected
    /// camera profile.
    ProfileCalibrationSignature(Byte) = 0xC6F4,

    /// A list of file offsets to extra Camera Profile IFDs.
    ///
    /// Note that the primary camera profile tags should be stored in IFD 0,
    /// and the [`Image::ExtraCameraProfiles`] tag should only be used if
    /// there is more than one camera profile stored in the DNG file.
    ExtraCameraProfiles(Long) = 0xC6F5,

    /// A UTF-8 encoded string containing the name of the "as shot" camera profile,
    /// if any.
    AsShotProfileName(Byte) = 0xC6F6,

    /// This tag indicates how much noise reduction has been applied to the raw data
    /// on a scale of `0.0` to `1.0`.
    ///
    /// - A `0.0` value indicates that no noise reduction has been applied.
    /// - A `1.0` value indicates that the "ideal" amount of noise reduction has
    ///   been applied, i.e. that the DNG reader should not apply additional noise
    ///   reduction by default.
    /// - A value of `0/0` indicates that this parameter is unknown.
    NoiseReductionApplied(Rational) = 0xC6F7,

    /// A UTF-8 encoded string containing the name of the camera profile.
    ///
    /// This tag is optional if there is only a single camera profile stored in
    /// the file but is required for all camera profiles if there is more than
    /// one camera profile stored in the file.
    ProfileName(Byte) = 0xC6F8,

    /// This tag specifies the number of input samples in each dimension of
    /// the hue/saturation/value mapping tables.
    ///
    /// The data for these tables are stored in [`Image::ProfileHueSatMapData1`],
    /// [`Image::ProfileHueSatMapData2`] and [`Image::ProfileHueSatMapData3`] tags.
    /// The most common case has `ValueDivisions` equal to `1`, so only hue and
    /// saturation are used as inputs to the mapping table.
    ProfileHueSatMapDims(Long) = 0xC6F9,

    /// This tag contains the data for the first hue/saturation/value mapping table.
    ///
    /// Each entry of the table contains three 32-bit IEEE floating-point values.
    /// The first entry is hue shift in degrees; the second entry is saturation
    /// scale factor; and the third entry is a value scale factor. The table entries
    /// are stored in the tag in nested loop order, with the value divisions in the
    /// outer loop, the hue divisions in the middle loop, and the saturation divisions
    /// in the inner loop. All zero input saturation entries are required to have a
    /// value scale factor of `1.0`.
    ProfileHueSatMapData1(Float) = 0xC6FA,

    /// This tag contains the data for the second hue/saturation/value mapping table.
    ///
    /// Each entry of the table contains three 32-bit IEEE floating-point values.
    /// The first entry is hue shift in degrees; the second entry is a saturation
    /// scale factor; and the third entry is a value scale factor. The table entries
    /// are stored in the tag in nested loop order, with the value divisions in
    /// the outer loop, the hue divisions in the middle loop, and the saturation
    /// divisions in the inner loop. All zero input saturation entries are required
    /// to have a value scale factor of `1.0`.
    ProfileHueSatMapData2(Float) = 0xC6FB,

    /// This tag contains a default tone curve that can be applied while processing
    /// the image as a starting point for user adjustments.
    ///
    /// The curve is specified as a list of 32-bit IEEE floating-point value pairs
    /// in linear gamma. Each sample has an input value in the range of `0.0` to `1.0`,
    /// and an output value in the range of `0.0` to `1.0`. The first sample is
    /// required to be `(0.0, 0.0)`, and the last sample is required to be `(1.0, 1.0)`.
    /// Interpolated the curve using a cubic spline.
    ProfileToneCurve(Float) = 0xC6FC,

    /// This tag contains information about the usage rules for the associated
    /// camera profile.
    ProfileEmbedPolicy(Long) = 0xC6FD,

    /// A UTF-8 encoded string containing the copyright information for the
    /// camera profile.
    ///
    /// This string always should be preserved along with the other camera profile tags.
    ProfileCopyright(Byte) = 0xC6FE,

    /// This tag defines a matrix that maps white balanced camera colors to XYZ D50
    /// colors.
    ForwardMatrix1(SRational) = 0xC714,

    /// This tag defines a matrix that maps white balanced camera colors to XYZ D50
    /// colors.
    ForwardMatrix2(SRational) = 0xC715,

    /// A UTF-8 encoded string containing the name of the application that created
    /// the preview stored in the IFD.
    PreviewApplicationName(Byte) = 0xC716,

    /// A UTF-8 encoded string containing the version number of the application
    /// that created the preview stored in the IFD.
    PreviewApplicationVersion(Byte) = 0xC717,

    /// A UTF-8 encoded string containing the name of the conversion settings
    /// (for example, snapshot name) used for the preview stored in the IFD.
    PreviewSettingsName(Byte) = 0xC718,

    /// A unique ID of the conversion settings (for example, MD5 digest) used to
    /// render the preview stored in the IFD.
    PreviewSettingsDigest(Byte) = 0xC719,

    /// This tag specifies the color space in which the rendered preview in this
    /// IFD is stored.
    ///
    /// The default value for this tag is sRGB for color previews and Gray Gamma 2.2
    /// for monochrome previews.
    PreviewColorSpace(Long) = 0xC71A,

    /// This tag is an [`Ascii`] string containing the name of the date/time at which
    /// the preview stored in the IFD was rendered.
    ///
    /// The date/time is encoded using `ISO 8601` format.
    PreviewDateTime(Ascii) = 0xC71B,

    /// This tag is an `MD5` digest of the raw image data.
    ///
    /// All pixels in the image are processed in row-scan order.
    /// Each pixel is zero padded to `16` or `32` bits deep (`16`-bit for data
    /// less than or equal to `16` bits deep, `32`-bit otherwise).
    /// The data for each pixel is processed in little-endian byte order.
    RawImageDigest(Undefined) = 0xC71C,

    /// This tag is an `MD5` digest of the data stored in the
    /// [`Image::OriginalRawFileData`] tag.
    OriginalRawFileDigest(Undefined) = 0xC71D,

    /// Normally, the pixels within a tile are stored in simple row-scan order.
    ///
    /// This tag specifies that the pixels within a tile should be grouped first
    /// into rectangular blocks of the specified size. These blocks are stored in
    /// row-scan order. Within each block, the pixels are stored in row-scan order.
    /// The use of a non-default value for this tag requires setting the
    /// [`Image::DNGBackwardVersion`] tag to at least `1.2.0.0`.
    SubTileBlockSize(Long) = 0xC71E,

    /// This tag specifies that rows of the image are stored in interleaved order.
    ///
    /// The value of the tag specifies the number of interleaved fields. The use of
    /// a non-default value for this tag requires setting the
    /// [`Image::DNGBackwardVersion`] tag to at least `1.2.0.0`.
    RowInterleaveFactor(Long) = 0xC71F,

    /// This tag specifies the number of input samples in each dimension of a default
    /// "look" table.
    ///
    /// The data for this table is stored in the [`Image::ProfileLookTableData`] tag.
    ProfileLookTableDims(Long) = 0xC725,

    /// This tag contains a default "look" table that can be applied while processing
    /// the image as a starting point for user adjustment.
    ///
    /// This table uses the same format as the tables stored in the
    /// [`Image::ProfileHueSatMapData1`] and [`Image::ProfileHueSatMapData2`] tags,
    /// and is applied in the same color space. However, it should be applied later
    /// in the processing pipe, after any exposure compensation and/or fill light
    /// stages, but before any tone curve stage. Each entry of the table contains
    /// three `32`-bit IEEE floating-point values. The first entry is hue shift in
    /// degrees, the second entry is a saturation scale factor, and the third entry
    /// is a value scale factor. The table entries are stored in the tag in nested
    /// loop order, with the value divisions in the outer loop, the hue divisions
    /// in the middle loop, and the saturation divisions in the inner loop. All zero
    /// input saturation entries are required to have a value scale factor of `1.0`.
    ProfileLookTableData(Float) = 0xC726,

    /// Specifies the list of opcodes that should be applied to the raw image,
    /// as read directly from the file.
    OpcodeList1(Undefined) = 0xC740,

    /// Specifies the list of opcodes that should be applied to the raw image,
    /// just after it has been mapped to linear reference values.
    OpcodeList2(Undefined) = 0xC741,

    /// Specifies the list of opcodes that should be applied to the raw image,
    /// just after it has been demosaiced.
    OpcodeList3(Undefined) = 0xC74E,

    /// [`Image::NoiseProfile`] describes the amount of noise in a raw image.
    ///
    /// Specifically, this tag models the amount of signal-dependent photon (shot)
    /// noise and signal-independent sensor readout noise, two common sources of
    /// noise in raw images. The model assumes that the noise is white and spatially
    /// independent, ignoring fixed pattern effects and other sources of noise
    /// (e.g., pixel response non-uniformity, spatially-dependent thermal effects, etc.).
    NoiseProfile(Double) = 0xC761,

    /// The optional [`Image::TimeCodes`] tag shall contain an ordered array of time codes.
    ///
    /// All time codes shall be `8` bytes long and in binary format. The tag may
    /// contain from `1` to `10` time codes. When the tag contains more than one
    /// time code, the first one shall be the default time code. This specification
    /// does not prescribe how to use multiple time codes. Each time code shall be
    /// as defined for the `8`-byte time code structure in SMPTE 331M-2004, Section 8.3.
    /// See also SMPTE 12-1-2008 and SMPTE 309-1999.
    TimeCodes(Byte) = 0xC763,

    /// The optional [`Image::FrameRate`] tag shall specify the video frame rate in
    /// number of image frames per second, expressed as a signed rational number.
    ///
    /// The numerator shall be non-negative and the denominator shall be positive.
    /// This field value is identical to the sample rate field in SMPTE 377-1-2009.
    FrameRate(SRational) = 0xC764,

    /// The optional [`Image::TStop`] tag shall specify the T-stop of the actual lens,
    /// expressed as an unsigned rational number.
    ///
    /// T-stop is also known as T-number or the photometric aperture of the lens.
    /// (F-number is the geometric aperture of the lens.) When the exact value is
    /// known, the T-stop shall be specified using a single number. Alternately,
    /// two numbers shall be used to indicate a T-stop range, in which case the
    /// first number shall be the minimum T-stop and the second number shall be
    /// the maximum T-stop.
    TStop(SRational) = 0xC772,

    /// The optional [`Image::ReelName`] tag shall specify a name for a sequence
    /// of images, where each image in the sequence has a unique image identifier
    /// (including but not limited to file name, frame number, date time, time code).
    ReelName(Ascii) = 0xC789,

    /// The optional [`Image::CameraLabel`] tag shall specify a text label for how
    /// the camera is used or assigned in this clip.
    ///
    /// This tag is similar to `CameraLabel` in XMP.
    CameraLabel(Ascii) = 0xC7A1,

    /// If this file is a proxy for a larger original DNG file, this tag specifics
    /// the default final size of the larger original file from which this proxy
    /// was generated.
    ///
    /// The default value for this tag is default final size of the current DNG file,
    /// which is [`Image::DefaultCropSize`] * [`Image::DefaultScale`].
    OriginalDefaultFinalSize(Long) = 0xC791,

    /// If this file is a proxy for a larger original DNG file, this tag specifics
    /// the best quality final size of the larger original file from which this
    /// proxy was generated.
    ///
    /// The default value for this tag is the [`Image::OriginalDefaultFinalSize`],
    /// if specified. Otherwise the default value for this tag is the best quality
    /// size of the current DNG file, which is
    /// [`Image::DefaultCropSize`] * [`Image::DefaultScale`] * [`Image::BestQualityScale`].
    OriginalBestQualityFinalSize(Long) = 0xC792,

    /// If this file is a proxy for a larger original DNG file, this tag specifics
    /// the [`Image::DefaultCropSize`] of the larger original file from which this
    /// proxy was generated.
    ///
    /// The default value for this tag is [`Image::OriginalDefaultFinalSize`],
    /// if specified. Otherwise, the default value for this tag is the
    /// [`Image::DefaultCropSize`] of the current DNG file.
    OriginalDefaultCropSize(Long) = 0xC793,

    /// Provides a way for color profiles to specify how indexing into a 3D
    /// `HueSatMap` is performed during raw conversion.
    ///
    /// This tag is not applicable to 2.5D `HueSatMap` tables (i.e., where the
    /// `Value` dimension is `1`).
    ProfileHueSatMapEncoding(Long) = 0xC7A3,

    /// Provides a way for color profiles to specify how indexing into a 3D
    /// `LookTable` is performed during raw conversion.
    ///
    /// This tag is not applicable to a 2.5D `LookTable` (i.e., where the
    /// `Value` dimension is `1`).
    ProfileLookTableEncoding(Long) = 0xC7A4,

    /// Provides a way for color profiles to increase or decrease exposure
    /// during raw conversion.
    ///
    /// [`Image::BaselineExposureOffset`] specifies the amount (in EV units) to
    /// add to the [`Image::BaselineExposure`] tag during image rendering. For
    /// example, if the [`Image::BaselineExposure`] value for a given camera model
    /// is `+0.3`, and the [`Image::BaselineExposureOffset`] value for a given
    /// camera profile used to render an image for that camera model is `-0.7`,
    /// then the actual default exposure value used during rendering will be
    /// `+0.3 - 0.7` = `-0.4`.
    BaselineExposureOffset(SRational) = 0xC7A5,

    /// This optional tag in a color profile provides a hint to the raw converter
    /// regarding how to handle the black point (e.g., flare subtraction) during
    /// rendering.
    ///
    /// If set to `Auto`, the raw converter should perform black subtraction during
    /// rendering. If set to `None`, the raw converter should not perform any black
    /// subtraction during rendering.
    DefaultBlackRender(Long) = 0xC7A6,

    /// This tag is a modified `MD5` digest of the raw image data.
    ///
    /// It has been updated from the algorithm used to compute the
    /// [`Image::RawImageDigest`] tag be more multi-processor friendly, and to
    /// support lossy compression algorithms.
    NewRawImageDigest(Byte) = 0xC7A7,

    /// The gain (what number the sample values are multiplied by) between the main
    /// raw IFD and the preview IFD containing this tag.
    RawToPreviewGain(Double) = 0xC7A8,

    /// Specifies a default user crop rectangle in relative coordinates.
    ///
    /// The values must satisfy:
    ///
    /// - `0.0` <= `top` < `bottom` <= `1.0`
    /// - `0.0` <= `left` < `right` <= `1.0`
    ///
    /// The default values of (`top` = `0`, `left` = `0`, `bottom` = `1`, `right` = `1`)
    /// correspond exactly to the default crop rectangle (as specified by the
    /// [`Image::DefaultCropOrigin`] and [`Image::DefaultCropSize`] tags).
    DefaultUserCrop(Rational) = 0xC7B5,

    /// Specifies the encoding of any depth data in the file.
    ///
    /// Can be unknown (apart from nearer distances being closer to zero, and
    /// farther distances being closer to the maximum value), linear (values
    /// vary linearly from zero representing [`Image::DepthNear`] to the maximum
    /// value representing [`Image::DepthFar`]), or inverse (values are stored
    /// inverse linearly, with zero representing [`Image::DepthNear`] and the
    /// maximum value representing [`Image::DepthFar`]).
    DepthFormat(Short) = 0xC7E9,

    /// Specifies distance from the camera represented by the zero value in the depth map.
    ///
    /// `0/0` means unknown.
    DepthNear(Rational) = 0xC7EA,

    /// Specifies distance from the camera represented by the maximum value in the depth map.
    ///
    /// - `0/0` means unknown
    /// - `1/0` means infinity, which is valid for unknown and inverse depth formats.
    DepthFar(Rational) = 0xC7EB,

    /// Specifies the measurement units for the [`Image::DepthNear`] and
    /// [`Image::DepthFar`] tags.
    DepthUnits(Short) = 0xC7EC,

    /// Specifies the measurement geometry for the depth map.
    ///
    /// Can be unknown, measured along the optical axis, or measured along the
    /// optical ray passing through each pixel.
    DepthMeasureType(Short) = 0xC7ED,

    /// A string that documents how the enhanced image data was processed.
    EnhanceParams(Ascii) = 0xC7EE,

    /// Contains spatially varying gain tables that can be applied while processing
    /// the image as a starting point for user adjustments.
    ProfileGainTableMap(Undefined) = 0xCD2D,

    /// A string that identifies the semantic mask.
    SemanticName(Ascii) = 0xCD2E,

    /// A string that identifies a specific instance in a semantic mask.
    SemanticInstanceID(Ascii) = 0xCD30,

    /// The illuminant used for an optional third set of color calibration tags
    /// ([`Image::ColorMatrix3`], [`Image::CameraCalibration3`], [`Image::ReductionMatrix3`]).
    ///
    /// The legal values for this tag are the same as the legal values for the
    /// [`Image::LightSource`] EXIF tag; [`Image::CalibrationIlluminant1`] and
    /// [`Image::CalibrationIlluminant2`] must also be present. If set to `255`
    /// (Other), then the IFD must also include a [`Image::IlluminantData3`] tag
    /// to specify the `x-y` chromaticity or spectral power distribution function
    /// for this illuminant.
    CalibrationIlluminant3(Short) = 0xCD31,

    /// [`Image::CameraCalibration3`] defines a calibration matrix that transforms
    /// reference camera native space values to individual camera native space
    /// values under the third calibration illuminant.
    ///
    /// The matrix is stored in row scan order. This matrix is stored separately
    /// from the matrix specified by the [`Image::ColorMatrix3`] tag to allow raw
    /// converters to swap in replacement color matrices based on
    /// [`Image::UniqueCameraModel`] tag, while still taking advantage of any
    /// per-individual camera calibration performed by the camera manufacturer.
    CameraCalibration3(SRational) = 0xCD32,

    /// [`Image::ColorMatrix3`] defines a transformation matrix that converts XYZ
    /// values to reference camera native color space values, under the third
    /// calibration illuminant.
    ///
    /// The matrix values are stored in row scan order.
    ColorMatrix3(SRational) = 0xCD33,

    /// This tag defines a matrix that maps white balanced camera colors to XYZ D50 colors.
    ForwardMatrix3(SRational) = 0xCD34,

    /// When the [`Image::CalibrationIlluminant1`] tag is set to `255` (Other), then
    /// the [`Image::IlluminantData1`] tag is required and specifies the data for the
    /// first illuminant.
    ///
    /// Otherwise, this tag is ignored. The illuminant data may be specified as either
    /// a `x-y` chromaticity coordinate or as a spectral power distribution function.
    IlluminantData1(Undefined) = 0xCD35,

    /// When the [`Image::CalibrationIlluminant2`] tag is set to `255` (Other), then
    /// the [`Image::IlluminantData2`] tag is required and specifies the data for the
    /// second illuminant.
    ///
    /// Otherwise, this tag is ignored. The format of the data is the same as
    /// [`Image::IlluminantData1`].
    IlluminantData2(Undefined) = 0xCD36,

    /// When the [`Image::CalibrationIlluminant3`] tag is set to `255` (Other), then
    /// the [`Image::IlluminantData3`] tag is required and specifies the data for the
    /// third illuminant.
    ///
    /// Otherwise, this tag is ignored. The format of the data is the same as
    /// [`Image::IlluminantData1`].
    IlluminantData3(Undefined) = 0xCD37,

    /// This tag identifies the crop rectangle of this IFD's mask, relative to the
    /// main image.
    MaskSubArea(Long) = 0xCD38,

    /// This tag contains the data for the third hue/saturation/value mapping table.
    ///
    /// Each entry of the table contains three `32`-bit IEEE floating-point values.
    /// The first entry is hue shift in degrees; the second entry is saturation scale
    /// factor; and the third entry is a value scale factor. The table entries are
    /// stored in the tag in nested loop order, with the value divisions in the outer
    /// loop, the hue divisions in the middle loop, and the saturation divisions in
    /// the inner loop. All zero input saturation entries are required to have a value
    /// scale factor of `1.0`.
    ProfileHueSatMapData3(Float) = 0xCD39,

    /// [`Image::ReductionMatrix3`] defines a dimensionality reduction matrix for use
    /// as the first stage in converting color camera native space values to XYZ values,
    /// under the third calibration illuminant.
    ///
    /// This tag may only be used if `ColorPlanes` is greater than `3`. The matrix is
    /// stored in row scan order.
    ReductionMatrix3(SRational) = 0xCD3A,

    /// This tag specifies color transforms that can be applied to masked image regions.
    ///
    /// Color transforms are specified using RGB-to-RGB color lookup tables.
    /// These tables are associated with Semantic Masks to limit the color transform
    /// to a sub-region of the image. The overall color transform is a linear
    /// combination of the color tables, weighted by their corresponding Semantic Masks.
    RGBTables(Undefined) = 0xCD3B,

    /// This tag is an extended version of [`Image::ProfileGainTableMap`].
    ProfileGainTableMap2(Undefined) = 0xCD40,

    /// This tag specifies that columns of the image are stored in interleaved order.
    ///
    /// The value of the tag specifies the number of interleaved fields.
    /// The use of a non-default value for this tag requires setting the
    /// [`Image::DNGBackwardVersion`] tag to at least `1.7.1.0`.
    ColumnInterleaveFactor(Long) = 0xCD43,

    /// This is an informative tag that describes how the image file relates to
    /// other image files captured in a sequence.
    ///
    /// Applications include focus stacking, merging multiple frames to reduce noise,
    /// time lapses, exposure brackets, stitched images for super resolution, and so on.
    ImageSequenceInfo(Undefined) = 0xCD44,

    /// This is an informative tag that provides basic statistical information about
    /// the pixel values of the image in this IFD.
    ///
    /// Possible applications include normalizing brightness of images when multiple
    /// images are displayed together (especially when mixing Standard Dynamic Range
    /// and High Dynamic Range images), identifying underexposed or overexposed images,
    /// and so on.
    ImageStats(Undefined) = 0xCD46,

    /// This tag describes the intended rendering output dynamic range for a given
    /// camera profile.
    ProfileDynamicRange(Undefined) = 0xCD47,

    /// A UTF-8 encoded string containing the 'group name' of the camera profile.
    ///
    /// The purpose of this tag is to associate two or more related camera profiles
    /// into a common group.
    ProfileGroupName(Ascii) = 0xCD48,

    /// This optional tag specifies the distance parameter used to encode the
    /// JPEG XL data in this IFD.
    ///
    /// A value of `0.0` means lossless compression, while values greater than
    /// `0.0` means lossy compression.
    JXLDistance(Float) = 0xCD49,

    /// This optional tag specifies the effort parameter used to encode the
    /// JPEG XL data in this IFD.
    ///
    /// Values range from `1` (low) to `9` (high).
    JXLEffort(Long) = 0xCD4A,

    /// This optional tag specifies the decode speed parameter used to encode the
    /// JPEG XL data in this IFD.
    ///
    /// Values range from `1` (slow) to `4` (fast).
    JXLDecodeSpeed(Long) = 0xCD4B,
}
