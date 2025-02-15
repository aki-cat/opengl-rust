#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cap {
    Blend,
    ClipDistance0,
    ClipDistance1,
    ClipDistance2,
    ClipDistance3,
    ClipDistance4,
    ClipDistance5,
    ClipDistance6,
    ClipDistance7,
    ColorLogicOp,
    CullFace,
    DebugOutput,
    DebugOutputSynchronous,
    DepthClamp,
    DepthTest,
    Dither,
    FramebufferSrgb,
    LineSmooth,
    Multisample,
    PolygonOffsetFill,
    PolygonOffsetLine,
    PolygonOffsetPoint,
    PolygonSmooth,
    PrimitiveRestart,
    PrimitiveRestartFixedIndex,
    RasterizerDiscard,
    SampleAlphaToCoverage,
    SampleAlphaToOne,
    SampleCoverage,
    SampleShading,
    SampleMask,
    ScissorTest,
    StencilTest,
    TextureCubeMapSeamless,
    ProgramPointSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Target {
    Array,
    AtomicCounter,
    CopyRead,
    CopyWrite,
    DispatchIndirect,
    DrawIndirect,
    ElementArray,
    PixelPack,
    PixelUnpack,
    Query,
    ShaderStorage,
    Texture,
    TransformFeedback,
    Uniform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Usage {
    StaticDraw,
    StaticRead,
    StaticCopy,
    StreamDraw,
    StreamRead,
    StreamCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy,
}

pub struct Mask;

impl Mask {
    pub const COLOR_BUFFER_BIT: u32 = gl::COLOR_BUFFER_BIT;
    pub const DEPTH_BUFFER_BIT: u32 = gl::DEPTH_BUFFER_BIT;
    pub const STENCIL_BUFFER_BIT: u32 = gl::STENCIL_BUFFER_BIT;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShaderType {
    Vertex,
    Fragmet,
    Geometry,
    TessControl,
    TessEvaluation,
    Compute,
}

use gl::types::{GLenum, GLfloat, GLint};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DepthMode {
    Component,
    Index,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompareFunc {
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Equal,
    NotEqual,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompareMode {
    CompareRefToTexture,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MinFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MagFilter {
    Nearest,
    Linear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Swizzle {
    Red,
    Green,
    Blue,
    Alpha,
    Zero,
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Wrap {
    ClampToEdge,
    ClampToBorder,
    MirroredRepeat,
    Repeat,
    MirrorClampToEdge,
}

pub enum TexParam {
    DepthStencilMode(DepthMode),
    BaseLevel(i32),
    CompareFunc(CompareFunc),
    CompareMode(CompareMode),
    LodBias(f32),
    MinFilter(MinFilter),
    MagFilter(MagFilter),
    MinLod(f32),
    MaxLod(f32),
    MaxLevel(i32),
    SwizzleR(Swizzle),
    SwizzleG(Swizzle),
    SwizzleB(Swizzle),
    SwizzleA(Swizzle),
    SwizzleRGBA(Swizzle, Swizzle, Swizzle, Swizzle),
    WrapS(Wrap),
    WrapT(Wrap),
    WrapR(Wrap),
}

pub(super) enum TexParamPair {
    GLf(GLenum, GLfloat),
    GLi(GLenum, GLint),
    GLiv(GLenum, [GLint; 4]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TexTarget {
    Tex1D,
    Tex2D,
    Tex3D,
    Tex1DArray,
    Tex2DArray,
    TexRectangle,
    TexCubeMap,
    TexCubeMapArray,
    TexBuffer,
    Tex2DMultisample,
    Tex2DMultisampleArray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MinmapTarget {
    Tex1D,
    Tex2D,
    Tex3D,
    Tex1DArray,
    Tex2DArray,
    TexCubeMap,
    TexCubeMapArray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageTarget {
    Tex2d,
    ProxyTex2d,
    Tex1dArray,
    ProxyTex1dArray,
    TexRectangle,
    ProxyTexRectangle,
    TexCubeMapPositiveX,
    TexCubeMapNegativeX,
    TexCubeMapPositiveY,
    TexCubeMapNegativeY,
    TexCubeMapPositiveZ,
    TexCubeMapNegativeZ,
    ProxyTexCubeMap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageFormat {
    Red,
    RG,
    RGB,
    BGR,
    RGBA,
    BGRA,
    DepthComponent,
    DepthStencil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BaseFormat {
    Red,
    RG,
    RGB,
    RGBA,
    DepthComponent,
    DepthStencil,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SizedFormat {
    R8,
    R8_SNORM,
    R16,
    R16_SNORM,
    RG8,
    RG8_SNORM,
    RG16,
    RG16_SNORM,
    R3_G3_B2,
    RGB4,
    RGB5,
    RGB8,
    RGB8_SNORM,
    RGB10,
    RGB12,
    RGB16_SNORM,
    RGBA2,
    RGBA4,
    RGB5_A1,
    RGBA8,
    RGBA8_SNORM,
    RGB10_A2,
    RGB10_A2UI,
    RGBA12,
    RGBA16,
    SRGB8,
    SRGB8_ALPHA8,
    R16F,
    RG16F,
    RGB16F,
    RGBA16F,
    R32F,
    RG32F,
    RGB32F,
    RGBA32F,
    R11F_G11F_B10F,
    RGB9_E5,
    R8I,
    R8UI,
    R16I,
    R16UI,
    R32I,
    R32UI,
    RG8I,
    RG8UI,
    RG16I,
    RG16UI,
    RG32I,
    RG32UI,
    RGB8I,
    RGB8UI,
    RGB16I,
    RGB16UI,
    RGB32I,
    RGB32UI,
    RGBA8I,
    RGBA8UI,
    RGBA16I,
    RGBA16UI,
    RGBA32I,
    RGBA32UI,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompressedFormat {
    RED,
    RG,
    RGB,
    RGBA,
    SRGB,
    SRGB_ALPHA,
    RED_RGTC1,
    SIGNED_RED_RGTC1,
    RG_RGTC2,
    SIGNED_RG_RGTC2,
    RGBA_BPTC_UNORM,
    SRGB_ALPHA_BPTC_UNORM,
    RGB_BPTC_SIGNED_FLOAT,
    RGB_BPTC_UNSIGNED_FLOAT,
}

pub enum InternalFormat {
    Base(BaseFormat),
    Sized(SizedFormat),
    Compressed(CompressedFormat),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PixelDataType {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    f32,
    HalfFloat,
    u8_3_3_2,
    u8_2_3_3_REV,
    u16_5_6_5,
    u16_5_6_5_REV,
    u16_4_4_4_4,
    u16_4_4_4_4_REV,
    u16_5_5_5_1,
    u16_1_5_5_5_REV,
    u32_8_8_8_8,
    u32_8_8_8_8_REV,
    u32_10_10_10_2,
    u32_2_10_10_10_REV,
}

pub type DepthFunc = CompareFunc;
pub type StencilFunc = CompareFunc;

impl TexParam {
    const fn swizzle(swizzle: Swizzle) -> i32 {
        match swizzle {
            Swizzle::Red => gl::RED as _,
            Swizzle::Green => gl::GREEN as _,
            Swizzle::Blue => gl::BLUE as _,
            Swizzle::Alpha => gl::ALPHA as _,
            Swizzle::Zero => gl::ZERO as _,
            Swizzle::One => gl::ONE as _,
        }
    }

    const fn wrap(wrap: Wrap) -> i32 {
        match wrap {
            Wrap::ClampToEdge => gl::CLAMP_TO_EDGE as _,
            Wrap::ClampToBorder => gl::CLAMP_TO_BORDER as _,
            Wrap::MirroredRepeat => gl::MIRRORED_REPEAT as _,
            Wrap::Repeat => gl::REPEAT as _,
            Wrap::MirrorClampToEdge => gl::MIRROR_CLAMP_TO_EDGE as _,
        }
    }

    #[inline]
    pub(super) const fn to_pair(&self) -> TexParamPair {
        match self {
            TexParam::DepthStencilMode(depth_mode) => match depth_mode {
                DepthMode::Component => {
                    TexParamPair::GLi(gl::DEPTH_STENCIL_TEXTURE_MODE, gl::DEPTH_COMPONENT as _)
                }
                DepthMode::Index => {
                    TexParamPair::GLi(gl::DEPTH_STENCIL_TEXTURE_MODE, gl::STENCIL_INDEX as _)
                }
            },
            TexParam::BaseLevel(level) => TexParamPair::GLi(gl::TEXTURE_BASE_LEVEL, *level),
            TexParam::CompareFunc(compare_func) => {
                TexParamPair::GLi(gl::TEXTURE_COMPARE_FUNC, compare_func.to_gl_func() as _)
            }
            TexParam::CompareMode(compare_mode) => match compare_mode {
                CompareMode::CompareRefToTexture => {
                    TexParamPair::GLi(gl::TEXTURE_COMPARE_MODE, gl::COMPARE_REF_TO_TEXTURE as _)
                }
                CompareMode::None => TexParamPair::GLi(gl::TEXTURE_COMPARE_MODE, gl::NONE as _),
            },
            TexParam::LodBias(value) => TexParamPair::GLf(gl::TEXTURE_LOD_BIAS, *value),
            TexParam::MinFilter(min_filter) => match min_filter {
                MinFilter::Nearest => TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::NEAREST as _),
                MinFilter::Linear => TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::LINEAR as _),
                MinFilter::NearestMipmapNearest => {
                    TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as _)
                }
                MinFilter::LinearMipmapNearest => {
                    TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as _)
                }
                MinFilter::NearestMipmapLinear => {
                    TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_LINEAR as _)
                }
                MinFilter::LinearMipmapLinear => {
                    TexParamPair::GLi(gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as _)
                }
            },
            TexParam::MagFilter(mag_filter) => match mag_filter {
                MagFilter::Nearest => TexParamPair::GLi(gl::TEXTURE_MAG_FILTER, gl::NEAREST as _),
                MagFilter::Linear => TexParamPair::GLi(gl::TEXTURE_MAG_FILTER, gl::LINEAR as _),
            },
            TexParam::MinLod(value) => TexParamPair::GLf(gl::TEXTURE_MIN_LOD, *value),
            TexParam::MaxLod(value) => TexParamPair::GLf(gl::TEXTURE_MAX_LOD, *value),
            TexParam::MaxLevel(value) => TexParamPair::GLi(gl::TEXTURE_MAX_LEVEL, *value),
            TexParam::SwizzleR(swizzle) => {
                TexParamPair::GLi(gl::TEXTURE_SWIZZLE_R, Self::swizzle(*swizzle))
            }
            TexParam::SwizzleG(swizzle) => {
                TexParamPair::GLi(gl::TEXTURE_SWIZZLE_G, Self::swizzle(*swizzle))
            }
            TexParam::SwizzleB(swizzle) => {
                TexParamPair::GLi(gl::TEXTURE_SWIZZLE_B, Self::swizzle(*swizzle))
            }
            TexParam::SwizzleA(swizzle) => {
                TexParamPair::GLi(gl::TEXTURE_SWIZZLE_A, Self::swizzle(*swizzle))
            }
            TexParam::SwizzleRGBA(sr, sg, sb, sa) => TexParamPair::GLiv(
                gl::TEXTURE_SWIZZLE_RGBA,
                [
                    Self::swizzle(*sr),
                    Self::swizzle(*sg),
                    Self::swizzle(*sb),
                    Self::swizzle(*sa),
                ],
            ),
            TexParam::WrapS(wrap) => TexParamPair::GLi(gl::TEXTURE_WRAP_S, Self::wrap(*wrap)),
            TexParam::WrapT(wrap) => TexParamPair::GLi(gl::TEXTURE_WRAP_T, Self::wrap(*wrap)),
            TexParam::WrapR(wrap) => TexParamPair::GLi(gl::TEXTURE_WRAP_R, Self::wrap(*wrap)),
        }
    }
}

impl TexTarget {
    #[inline]
    pub(super) const fn to_gl_target(&self) -> u32 {
        match self {
            TexTarget::Tex1D => gl::TEXTURE_1D,
            TexTarget::Tex2D => gl::TEXTURE_2D,
            TexTarget::Tex3D => gl::TEXTURE_3D,
            TexTarget::Tex1DArray => gl::TEXTURE_1D_ARRAY,
            TexTarget::Tex2DArray => gl::TEXTURE_1D_ARRAY,
            TexTarget::TexRectangle => gl::TEXTURE_RECTANGLE,
            TexTarget::TexCubeMap => gl::TEXTURE_CUBE_MAP,
            TexTarget::TexCubeMapArray => gl::TEXTURE_CUBE_MAP_ARRAY,
            TexTarget::TexBuffer => gl::TEXTURE_BUFFER,
            TexTarget::Tex2DMultisample => gl::TEXTURE_2D_MULTISAMPLE,
            TexTarget::Tex2DMultisampleArray => gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
        }
    }
}

impl MinmapTarget {
    #[allow(dead_code)]
    #[inline]
    pub(super) const fn to_gl_target(&self) -> u32 {
        match self {
            MinmapTarget::Tex1D => gl::TEXTURE_1D,
            MinmapTarget::Tex2D => gl::TEXTURE_2D,
            MinmapTarget::Tex3D => gl::TEXTURE_3D,
            MinmapTarget::Tex1DArray => gl::TEXTURE_1D_ARRAY,
            MinmapTarget::Tex2DArray => gl::TEXTURE_1D_ARRAY,
            MinmapTarget::TexCubeMap => gl::TEXTURE_CUBE_MAP,
            MinmapTarget::TexCubeMapArray => gl::TEXTURE_CUBE_MAP_ARRAY,
        }
    }
}

impl Target {
    #[inline]
    pub(super) const fn to_gl_target(self) -> GLenum {
        match self {
            Target::Array => gl::ARRAY_BUFFER,
            Target::AtomicCounter => gl::ATOMIC_COUNTER_BUFFER,
            Target::CopyRead => gl::COPY_READ_BUFFER,
            Target::CopyWrite => gl::COPY_WRITE_BUFFER,
            Target::DispatchIndirect => gl::DISPATCH_INDIRECT_BUFFER,
            Target::DrawIndirect => gl::DRAW_INDIRECT_BUFFER,
            Target::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
            Target::PixelPack => gl::PIXEL_PACK_BUFFER,
            Target::PixelUnpack => gl::PIXEL_UNPACK_BUFFER,
            Target::Query => gl::QUERY_BUFFER,
            Target::ShaderStorage => gl::SHADER_STORAGE_BUFFER,
            Target::Texture => gl::TEXTURE_BUFFER,
            Target::TransformFeedback => gl::TRANSFORM_FEEDBACK_BUFFER,
            Target::Uniform => gl::UNIFORM_BUFFER,
        }
    }
}

impl Usage {
    #[inline]
    pub(super) const fn to_gl_usage(self) -> GLenum {
        match self {
            Usage::StaticDraw => gl::STATIC_DRAW,
            Usage::StaticRead => gl::STATIC_READ,
            Usage::StaticCopy => gl::STATIC_COPY,
            Usage::StreamDraw => gl::STREAM_DRAW,
            Usage::StreamRead => gl::STREAM_READ,
            Usage::StreamCopy => gl::STREAM_COPY,
            Usage::DynamicDraw => gl::DYNAMIC_DRAW,
            Usage::DynamicRead => gl::DYNAMIC_READ,
            Usage::DynamicCopy => gl::DYNAMIC_COPY,
        }
    }
}

impl Mode {
    pub(crate) const fn to_gl_mode(self) -> u32 {
        match self {
            Mode::Points => gl::POINTS,
            Mode::LineStrip => gl::LINE_STRIP,
            Mode::LineLoop => gl::LINE_LOOP,
            Mode::Lines => gl::LINES,
            Mode::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            Mode::LinesAdjacency => gl::LINES_ADJACENCY,
            Mode::TriangleStrip => gl::TRIANGLE_STRIP,
            Mode::TriangleFan => gl::TRIANGLE_FAN,
            Mode::Triangles => gl::TRIANGLES,
            Mode::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            Mode::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            Mode::Patches => gl::PATCHES,
        }
    }
}

impl Cap {
    pub(crate) const fn to_gl_cap(self) -> u32 {
        match self {
            Cap::Blend => gl::BLEND,
            Cap::ClipDistance0 => gl::CLIP_DISTANCE0,
            Cap::ClipDistance1 => gl::CLIP_DISTANCE1,
            Cap::ClipDistance2 => gl::CLIP_DISTANCE2,
            Cap::ClipDistance3 => gl::CLIP_DISTANCE3,
            Cap::ClipDistance4 => gl::CLIP_DISTANCE4,
            Cap::ClipDistance5 => gl::CLIP_DISTANCE5,
            Cap::ClipDistance6 => gl::CLIP_DISTANCE6,
            Cap::ClipDistance7 => gl::CLIP_DISTANCE7,
            Cap::ColorLogicOp => gl::COLOR_LOGIC_OP,
            Cap::CullFace => gl::CULL_FACE,
            Cap::DebugOutput => gl::DEBUG_OUTPUT,
            Cap::DebugOutputSynchronous => gl::DEBUG_OUTPUT_SYNCHRONOUS,
            Cap::DepthClamp => gl::DEPTH_CLAMP,
            Cap::DepthTest => gl::DEPTH_TEST,
            Cap::Dither => gl::DITHER,
            Cap::FramebufferSrgb => gl::FRAMEBUFFER_SRGB,
            Cap::LineSmooth => gl::LINE_SMOOTH,
            Cap::Multisample => gl::MULTISAMPLE,
            Cap::PolygonOffsetFill => gl::POLYGON_OFFSET_FILL,
            Cap::PolygonOffsetLine => gl::POLYGON_OFFSET_LINE,
            Cap::PolygonOffsetPoint => gl::POLYGON_OFFSET_POINT,
            Cap::PolygonSmooth => gl::POLYGON_SMOOTH,
            Cap::PrimitiveRestart => gl::PRIMITIVE_RESTART,
            Cap::PrimitiveRestartFixedIndex => gl::PRIMITIVE_RESTART_FIXED_INDEX,
            Cap::RasterizerDiscard => gl::RASTERIZER_DISCARD,
            Cap::SampleAlphaToCoverage => gl::SAMPLE_ALPHA_TO_COVERAGE,
            Cap::SampleAlphaToOne => gl::SAMPLE_ALPHA_TO_ONE,
            Cap::SampleCoverage => gl::SAMPLE_COVERAGE,
            Cap::SampleShading => gl::SAMPLE_SHADING,
            Cap::SampleMask => gl::SAMPLE_MASK,
            Cap::ScissorTest => gl::SCISSOR_TEST,
            Cap::StencilTest => gl::STENCIL_TEST,
            Cap::TextureCubeMapSeamless => gl::TEXTURE_CUBE_MAP_SEAMLESS,
            Cap::ProgramPointSize => gl::PROGRAM_POINT_SIZE,
        }
    }
}

impl ShaderType {
    pub(crate) const fn to_gl_type(self) -> GLenum {
        match self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragmet => gl::FRAGMENT_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::TessControl => gl::TESS_CONTROL_SHADER,
            ShaderType::TessEvaluation => gl::TESS_EVALUATION_SHADER,
            ShaderType::Compute => gl::COMPUTE_SHADER,
        }
    }
}

impl ImageTarget {
    #[inline]
    pub(super) const fn to_gl_target(self) -> GLenum {
        match self {
            ImageTarget::Tex2d => gl::TEXTURE_2D,
            ImageTarget::ProxyTex2d => gl::PROXY_TEXTURE_2D,
            ImageTarget::Tex1dArray => gl::TEXTURE_1D_ARRAY,
            ImageTarget::ProxyTex1dArray => gl::PROXY_TEXTURE_1D_ARRAY,
            ImageTarget::TexRectangle => gl::TEXTURE_RECTANGLE,
            ImageTarget::ProxyTexRectangle => gl::PROXY_TEXTURE_RECTANGLE,
            ImageTarget::TexCubeMapPositiveX => gl::TEXTURE_CUBE_MAP_POSITIVE_X,
            ImageTarget::TexCubeMapNegativeX => gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
            ImageTarget::TexCubeMapPositiveY => gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
            ImageTarget::TexCubeMapNegativeY => gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            ImageTarget::TexCubeMapPositiveZ => gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
            ImageTarget::TexCubeMapNegativeZ => gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
            ImageTarget::ProxyTexCubeMap => gl::PROXY_TEXTURE_CUBE_MAP,
        }
    }
}

impl BaseFormat {
    #[inline]
    pub(super) const fn to_gl_format(self) -> GLenum {
        match self {
            BaseFormat::Red => gl::RED,
            BaseFormat::RG => gl::RG,
            BaseFormat::RGB => gl::RGB,
            BaseFormat::RGBA => gl::RGBA,
            BaseFormat::DepthComponent => gl::DEPTH_COMPONENT,
            BaseFormat::DepthStencil => gl::DEPTH_STENCIL,
        }
    }
}

impl SizedFormat {
    #[inline]
    pub(super) const fn to_gl_format(self) -> GLenum {
        match self {
            SizedFormat::R8 => gl::R8,
            SizedFormat::R8_SNORM => gl::R8_SNORM,
            SizedFormat::R16 => gl::R16,
            SizedFormat::R16_SNORM => gl::R16_SNORM,
            SizedFormat::RG8 => gl::RG8,
            SizedFormat::RG8_SNORM => gl::RG8_SNORM,
            SizedFormat::RG16 => gl::RG16,
            SizedFormat::RG16_SNORM => gl::RG16_SNORM,
            SizedFormat::R3_G3_B2 => gl::R3_G3_B2,
            SizedFormat::RGB4 => gl::RGB4,
            SizedFormat::RGB5 => gl::RGB5,
            SizedFormat::RGB8 => gl::RGB8,
            SizedFormat::RGB8_SNORM => gl::RGB8_SNORM,
            SizedFormat::RGB10 => gl::RGB10,
            SizedFormat::RGB12 => gl::RGB12,
            SizedFormat::RGB16_SNORM => gl::RGB16_SNORM,
            SizedFormat::RGBA2 => gl::RGBA2,
            SizedFormat::RGBA4 => gl::RGBA4,
            SizedFormat::RGB5_A1 => gl::RGB5_A1,
            SizedFormat::RGBA8 => gl::RGBA8,
            SizedFormat::RGBA8_SNORM => gl::RGBA8_SNORM,
            SizedFormat::RGB10_A2 => gl::RGB10_A2,
            SizedFormat::RGB10_A2UI => gl::RGB10_A2UI,
            SizedFormat::RGBA12 => gl::RGBA12,
            SizedFormat::RGBA16 => gl::RGBA16,
            SizedFormat::SRGB8 => gl::SRGB8,
            SizedFormat::SRGB8_ALPHA8 => gl::SRGB8_ALPHA8,
            SizedFormat::R16F => gl::R16F,
            SizedFormat::RG16F => gl::RG16F,
            SizedFormat::RGB16F => gl::RGB16F,
            SizedFormat::RGBA16F => gl::RGBA16F,
            SizedFormat::R32F => gl::R32F,
            SizedFormat::RG32F => gl::RG32F,
            SizedFormat::RGB32F => gl::RGB32F,
            SizedFormat::RGBA32F => gl::RGBA32F,
            SizedFormat::R11F_G11F_B10F => gl::R11F_G11F_B10F,
            SizedFormat::RGB9_E5 => gl::RGB9_E5,
            SizedFormat::R8I => gl::R8I,
            SizedFormat::R8UI => gl::R8UI,
            SizedFormat::R16I => gl::R16I,
            SizedFormat::R16UI => gl::R16UI,
            SizedFormat::R32I => gl::R32I,
            SizedFormat::R32UI => gl::R32UI,
            SizedFormat::RG8I => gl::RG8I,
            SizedFormat::RG8UI => gl::RG8UI,
            SizedFormat::RG16I => gl::RG16I,
            SizedFormat::RG16UI => gl::RG16UI,
            SizedFormat::RG32I => gl::RG32I,
            SizedFormat::RG32UI => gl::RG32UI,
            SizedFormat::RGB8I => gl::RGB8I,
            SizedFormat::RGB8UI => gl::RGB8UI,
            SizedFormat::RGB16I => gl::RGB16I,
            SizedFormat::RGB16UI => gl::RGB16UI,
            SizedFormat::RGB32I => gl::RGB32I,
            SizedFormat::RGB32UI => gl::RGB32UI,
            SizedFormat::RGBA8I => gl::RGBA8I,
            SizedFormat::RGBA8UI => gl::RGBA8UI,
            SizedFormat::RGBA16I => gl::RGBA16I,
            SizedFormat::RGBA16UI => gl::RGBA16UI,
            SizedFormat::RGBA32I => gl::RGBA32I,
            SizedFormat::RGBA32UI => gl::RGBA32UI,
        }
    }
}

impl CompressedFormat {
    #[inline]
    pub(super) const fn to_gl_format(self) -> GLenum {
        match self {
            CompressedFormat::RED => gl::COMPRESSED_RED,
            CompressedFormat::RG => gl::COMPRESSED_RG,
            CompressedFormat::RGB => gl::COMPRESSED_RGB,
            CompressedFormat::RGBA => gl::COMPRESSED_RGBA,
            CompressedFormat::SRGB => gl::COMPRESSED_SRGB,
            CompressedFormat::SRGB_ALPHA => gl::COMPRESSED_SRGB_ALPHA,
            CompressedFormat::RED_RGTC1 => gl::COMPRESSED_RED_RGTC1,
            CompressedFormat::SIGNED_RED_RGTC1 => gl::COMPRESSED_SIGNED_RED_RGTC1,
            CompressedFormat::RG_RGTC2 => gl::COMPRESSED_RG_RGTC2,
            CompressedFormat::SIGNED_RG_RGTC2 => gl::COMPRESSED_SIGNED_RG_RGTC2,
            CompressedFormat::RGBA_BPTC_UNORM => gl::COMPRESSED_RGBA_BPTC_UNORM,
            CompressedFormat::SRGB_ALPHA_BPTC_UNORM => gl::COMPRESSED_SRGB_ALPHA_BPTC_UNORM,
            CompressedFormat::RGB_BPTC_SIGNED_FLOAT => gl::COMPRESSED_RGB_BPTC_SIGNED_FLOAT,
            CompressedFormat::RGB_BPTC_UNSIGNED_FLOAT => gl::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT,
        }
    }
}

impl InternalFormat {
    #[inline]
    pub(super) const fn to_gl_format(self) -> GLenum {
        match self {
            InternalFormat::Base(base_format) => base_format.to_gl_format(),
            InternalFormat::Sized(sized_format) => sized_format.to_gl_format(),
            InternalFormat::Compressed(compressed_format) => compressed_format.to_gl_format(),
        }
    }
}

impl ImageFormat {
    #[inline]
    pub(super) const fn to_gl_format(self) -> GLenum {
        match self {
            ImageFormat::Red => gl::RED,
            ImageFormat::RG => gl::RG,
            ImageFormat::RGB => gl::RGB,
            ImageFormat::BGR => gl::BGR,
            ImageFormat::RGBA => gl::RGBA,
            ImageFormat::BGRA => gl::BGRA,
            ImageFormat::DepthComponent => gl::DEPTH_COMPONENT,
            ImageFormat::DepthStencil => gl::DEPTH_STENCIL,
        }
    }
}

impl PixelDataType {
    #[inline]
    pub(super) const fn to_gl_type(self) -> GLenum {
        match self {
            PixelDataType::u8 => gl::UNSIGNED_BYTE,
            PixelDataType::i8 => gl::BYTE,
            PixelDataType::u16 => gl::UNSIGNED_SHORT,
            PixelDataType::i16 => gl::SHORT,
            PixelDataType::u32 => gl::UNSIGNED_INT,
            PixelDataType::i32 => gl::INT,
            PixelDataType::f32 => gl::FLOAT,
            PixelDataType::HalfFloat => gl::HALF_FLOAT,
            PixelDataType::u8_3_3_2 => gl::UNSIGNED_BYTE_3_3_2,
            PixelDataType::u8_2_3_3_REV => gl::UNSIGNED_BYTE_2_3_3_REV,
            PixelDataType::u16_5_6_5 => gl::UNSIGNED_SHORT_5_6_5,
            PixelDataType::u16_5_6_5_REV => gl::UNSIGNED_SHORT_5_6_5_REV,
            PixelDataType::u16_4_4_4_4 => gl::UNSIGNED_SHORT_4_4_4_4,
            PixelDataType::u16_4_4_4_4_REV => gl::UNSIGNED_SHORT_4_4_4_4_REV,
            PixelDataType::u16_5_5_5_1 => gl::UNSIGNED_SHORT_5_5_5_1,
            PixelDataType::u16_1_5_5_5_REV => gl::UNSIGNED_SHORT_1_5_5_5_REV,
            PixelDataType::u32_8_8_8_8 => gl::UNSIGNED_INT_8_8_8_8,
            PixelDataType::u32_8_8_8_8_REV => gl::UNSIGNED_INT_8_8_8_8_REV,
            PixelDataType::u32_10_10_10_2 => gl::UNSIGNED_INT_10_10_10_2,
            PixelDataType::u32_2_10_10_10_REV => gl::UNSIGNED_INT_2_10_10_10_REV,
        }
    }
}

impl CompareFunc {
    #[inline]
    pub(super) const fn to_gl_func(self) -> GLenum {
        match self {
            CompareFunc::Never => gl::NEVER,
            CompareFunc::Less => gl::LESS,
            CompareFunc::Equal => gl::EQUAL,
            CompareFunc::LessEqual => gl::LEQUAL,
            CompareFunc::Greater => gl::GREATER,
            CompareFunc::NotEqual => gl::NOTEQUAL,
            CompareFunc::GreaterEqual => gl::GEQUAL,
            CompareFunc::Always => gl::ALWAYS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StencilOp {
    Keep,
    Zero,
    Replace,
    Incr,
    IncrWrap,
    Decr,
    DecrWrap,
    Invert,
}

impl StencilOp {
    #[inline]
    pub(super) const fn to_gl_op(self) -> GLenum {
        match self {
            StencilOp::Keep => gl::KEEP,
            StencilOp::Zero => gl::ZERO,
            StencilOp::Replace => gl::REPLACE,
            StencilOp::Incr => gl::INCR,
            StencilOp::IncrWrap => gl::INCR_WRAP,
            StencilOp::Decr => gl::DECR,
            StencilOp::DecrWrap => gl::DECR_WRAP,
            StencilOp::Invert => gl::INVERT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
}

impl BlendFactor {
    #[inline]
    pub(super) const fn to_gl_func(self) -> GLenum {
        match self {
            BlendFactor::Zero => gl::ZERO,
            BlendFactor::One => gl::ONE,
            BlendFactor::SrcColor => gl::SRC_COLOR,
            BlendFactor::OneMinusSrcColor => gl::ONE_MINUS_SRC_COLOR,
            BlendFactor::DstColor => gl::DST_COLOR,
            BlendFactor::OneMinusDstColor => gl::ONE_MINUS_DST_COLOR,
            BlendFactor::SrcAlpha => gl::SRC_ALPHA,
            BlendFactor::OneMinusSrcAlpha => gl::ONE_MINUS_SRC_ALPHA,
            BlendFactor::DstAlpha => gl::DST_ALPHA,
            BlendFactor::OneMinusDstAlpha => gl::ONE_MINUS_DST_ALPHA,
            BlendFactor::ConstantColor => gl::CONSTANT_COLOR,
            BlendFactor::OneMinusConstantColor => gl::ONE_MINUS_CONSTANT_COLOR,
            BlendFactor::ConstantAlpha => gl::CONSTANT_ALPHA,
            BlendFactor::OneMinusConstantAlpha => gl::ONE_MINUS_CONSTANT_ALPHA,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlendEquation {
    FuncAdd,
    FuncSubtract,
    FuncReverseSubtract,
    Min,
    Max,
}

impl BlendEquation {
    #[inline]
    pub(super) const fn to_gl_equation(self) -> GLenum {
        match self {
            BlendEquation::FuncAdd => gl::FUNC_ADD,
            BlendEquation::FuncSubtract => gl::FUNC_SUBTRACT,
            BlendEquation::FuncReverseSubtract => gl::FUNC_REVERSE_SUBTRACT,
            BlendEquation::Min => gl::MIN,
            BlendEquation::Max => gl::MAX,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CullFace {
    Front,
    Back,
    FrontAndBack,
}

impl CullFace {
    #[inline]
    pub(super) const fn to_gl_face(self) -> GLenum {
        match self {
            CullFace::Front => gl::FRONT,
            CullFace::Back => gl::BACK,
            CullFace::FrontAndBack => gl::FRONT_AND_BACK,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FrontFace {
    Clockwise,
    CounterClockwise,
}

impl FrontFace {
    #[inline]
    pub(super) const fn to_gl_face(self) -> GLenum {
        match self {
            FrontFace::Clockwise => gl::CW,
            FrontFace::CounterClockwise => gl::CCW,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FrameBufferTarget {
    Read,
    Draw,
    ReadDraw,
}

impl FrameBufferTarget {
    #[inline]
    pub(super) const fn to_gl_target(self) -> GLenum {
        match self {
            FrameBufferTarget::Read => gl::READ_FRAMEBUFFER,
            FrameBufferTarget::Draw => gl::DRAW_FRAMEBUFFER,
            FrameBufferTarget::ReadDraw => gl::READ_FRAMEBUFFER,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FrameBufferAttachment {
    Color,
    Depth,
    Stencil,
    DepthStencil,
}

impl FrameBufferAttachment {
    #[inline]
    pub(super) const fn to_gl_attachment(self) -> GLenum {
        match self {
            FrameBufferAttachment::Color => gl::COLOR_ATTACHMENT0,
            FrameBufferAttachment::Depth => gl::DEPTH_ATTACHMENT,
            FrameBufferAttachment::Stencil => gl::STENCIL_ATTACHMENT,
            FrameBufferAttachment::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextureTarget {
    Tex2d,
    Tex2dMultisample,
    TexCubeMapPositiveX,
    TexCubeMapNegativeX,
    TexCubeMapPositiveY,
    TexCubeMapNegativeY,
    TexCubeMapPositiveZ,
    TexCubeMapNegativeZ,
}

impl TextureTarget {
    #[inline]
    pub(super) const fn to_gl_enum(self) -> GLenum {
        match self {
            TextureTarget::Tex2d => gl::TEXTURE_2D,
            TextureTarget::Tex2dMultisample => gl::TEXTURE_2D_MULTISAMPLE,
            TextureTarget::TexCubeMapPositiveX => gl::TEXTURE_CUBE_MAP_POSITIVE_X,
            TextureTarget::TexCubeMapNegativeX => gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
            TextureTarget::TexCubeMapPositiveY => gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
            TextureTarget::TexCubeMapNegativeY => gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            TextureTarget::TexCubeMapPositiveZ => gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
            TextureTarget::TexCubeMapNegativeZ => gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum RenderBufferFormat {
    R8,
    R8UI,
    R8I,
    R16UI,
    R16I,
    R32UI,
    R32I,
    RG8,
    RG8UI,
    RG8I,
    RG16UI,
    RG16I,
    RG32UI,
    RG32I,
    RGB8,
    RGB565,
    RGBA8,
    SRGB8_ALPHA8,
    RGB5_A1,
    RGBA4,
    RGB10_A2,
    RGBA8UI,
    RGBA8I,
    RGB10_A2UI,
    RGBA16UI,
    RGBA16I,
    RGBA32I,
    RGBA32UI,
    DEPTH_COMPONENT16,
    DEPTH_COMPONENT24,
    DEPTH_COMPONENT32F,
    DEPTH24_STENCIL8,
    DEPTH32F_STENCIL8,
    STENCIL_INDEX,
}

impl RenderBufferFormat {
    #[inline]
    pub(super) const fn to_gl_format(self) -> GLenum {
        match self {
            RenderBufferFormat::R8 => gl::R8,
            RenderBufferFormat::R8UI => gl::R8UI,
            RenderBufferFormat::R8I => gl::R8I,
            RenderBufferFormat::R16UI => gl::R16UI,
            RenderBufferFormat::R16I => gl::R16I,
            RenderBufferFormat::R32UI => gl::R32UI,
            RenderBufferFormat::R32I => gl::R32I,
            RenderBufferFormat::RG8 => gl::RG8,
            RenderBufferFormat::RG8UI => gl::RG8UI,
            RenderBufferFormat::RG8I => gl::RG8I,
            RenderBufferFormat::RG16UI => gl::RG16UI,
            RenderBufferFormat::RG16I => gl::RG16I,
            RenderBufferFormat::RG32UI => gl::RG32UI,
            RenderBufferFormat::RG32I => gl::RG32I,
            RenderBufferFormat::RGB8 => gl::RGB8,
            RenderBufferFormat::RGB565 => gl::RGB565,
            RenderBufferFormat::RGBA8 => gl::RGBA8,
            RenderBufferFormat::SRGB8_ALPHA8 => gl::SRGB8_ALPHA8,
            RenderBufferFormat::RGB5_A1 => gl::RGB5_A1,
            RenderBufferFormat::RGBA4 => gl::RGBA4,
            RenderBufferFormat::RGB10_A2 => gl::RGB10_A2,
            RenderBufferFormat::RGBA8UI => gl::RGBA8UI,
            RenderBufferFormat::RGBA8I => gl::RGBA8I,
            RenderBufferFormat::RGB10_A2UI => gl::RGB10_A2UI,
            RenderBufferFormat::RGBA16UI => gl::RGBA16UI,
            RenderBufferFormat::RGBA16I => gl::RGBA16I,
            RenderBufferFormat::RGBA32I => gl::RGBA32I,
            RenderBufferFormat::RGBA32UI => gl::RGBA32UI,
            RenderBufferFormat::DEPTH_COMPONENT16 => gl::DEPTH_COMPONENT16,
            RenderBufferFormat::DEPTH_COMPONENT24 => gl::DEPTH_COMPONENT24,
            RenderBufferFormat::DEPTH_COMPONENT32F => gl::DEPTH_COMPONENT32F,
            RenderBufferFormat::DEPTH24_STENCIL8 => gl::DEPTH24_STENCIL8,
            RenderBufferFormat::DEPTH32F_STENCIL8 => gl::DEPTH32F_STENCIL8,
            RenderBufferFormat::STENCIL_INDEX => gl::STENCIL_INDEX,
        }
    }
}

pub enum Attachmect {
    Color(u32),
    Depth,
    Stencil,
    DepthStencil,
}

impl Attachmect {
    #[inline]
    pub(super) const fn to_gl_attachment(self) -> GLenum {
        match self {
            Attachmect::Color(index) => gl::COLOR_ATTACHMENT0 + index,
            Attachmect::Depth => gl::DEPTH_ATTACHMENT,
            Attachmect::Stencil => gl::STENCIL_ATTACHMENT,
            Attachmect::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GlType {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    f32,
    f64,
}

impl GlType {
    #[inline]
    pub(super) const fn size(self) -> usize {
        match self {
            GlType::u8 => std::mem::size_of::<u8>(),
            GlType::i8 => std::mem::size_of::<i8>(),
            GlType::u16 => std::mem::size_of::<u16>(),
            GlType::i16 => std::mem::size_of::<i16>(),
            GlType::u32 => std::mem::size_of::<u32>(),
            GlType::i32 => std::mem::size_of::<i32>(),
            GlType::f32 => std::mem::size_of::<f32>(),
            GlType::f64 => std::mem::size_of::<f64>(),
        }
    }

    #[inline]
    pub(super) const fn to_gl_type(self) -> GLenum {
        match self {
            GlType::u8 => gl::UNSIGNED_BYTE,
            GlType::i8 => gl::BYTE,
            GlType::u16 => gl::UNSIGNED_SHORT,
            GlType::i16 => gl::SHORT,
            GlType::u32 => gl::UNSIGNED_INT,
            GlType::i32 => gl::INT,
            GlType::f32 => gl::FLOAT,
            GlType::f64 => gl::DOUBLE,
        }
    }
}

pub struct TexCubeMap {
    index: u8,
}

impl TexCubeMap {
    #[inline]
    pub const fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for TexCubeMap {
    type Item = ImageTarget;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index {
            1 => Some(ImageTarget::TexCubeMapPositiveX),
            2 => Some(ImageTarget::TexCubeMapNegativeX),
            3 => Some(ImageTarget::TexCubeMapPositiveY),
            4 => Some(ImageTarget::TexCubeMapNegativeY),
            5 => Some(ImageTarget::TexCubeMapPositiveZ),
            6 => Some(ImageTarget::TexCubeMapNegativeZ),
            _ => return None,
        }
    }
}

pub enum Filter {
    Nearest,
    Linear,
}

impl Filter {
    pub(super) const fn to_gl_filter(self) -> GLenum {
        match self {
            Filter::Nearest => gl::NEAREST,
            Filter::Linear => gl::LINEAR,
        }
    }
}
