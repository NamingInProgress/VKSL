use itertools::Itertools;
use mvutils_proc_macro::TryFromString;
use crate::scope::ast::Literal;
use crate::scope::ast::ty::Type;

#[derive(Copy, Clone, Debug, PartialEq, TryFromString)]
pub enum BuiltinFn {
    // Scalar / vector constructors
    #[casing(Lower)]
    Bool,
    #[casing(Lower)]
    Int,
    #[casing(Lower)]
    Uint,
    #[casing(Lower)]
    F32,
    #[casing(Lower)]
    F64,

    #[casing(Lower)]
    BVec2,
    #[casing(Lower)]
    BVec3,
    #[casing(Lower)]
    BVec4,

    #[casing(Lower)]
    IVec2,
    #[casing(Lower)]
    IVec3,
    #[casing(Lower)]
    IVec4,

    #[casing(Lower)]
    UVec2,
    #[casing(Lower)]
    UVec3,
    #[casing(Lower)]
    UVec4,

    #[casing(Lower)]
    Vec2,
    #[casing(Lower)]
    Vec3,
    #[casing(Lower)]
    Vec4,

    #[casing(Lower)]
    DVec2,
    #[casing(Lower)]
    DVec3,
    #[casing(Lower)]
    DVec4,

    // Matrix constructors
    #[casing(Lower)]
    Mat2,
    #[casing(Lower)]
    Mat3,
    #[casing(Lower)]
    Mat4,
    #[casing(Lower)]
    Mat2x2,
    #[casing(Lower)]
    Mat2x3,
    #[casing(Lower)]
    Mat2x4,
    #[casing(Lower)]
    Mat3x2,
    #[casing(Lower)]
    Mat3x3,
    #[casing(Lower)]
    Mat3x4,
    #[casing(Lower)]
    Mat4x2,
    #[casing(Lower)]
    Mat4x3,
    #[casing(Lower)]
    Mat4x4,

    #[casing(Lower)]
    DMat2,
    #[casing(Lower)]
    DMat3,
    #[casing(Lower)]
    DMat4,
    #[casing(Lower)]
    DMat2x2,
    #[casing(Lower)]
    DMat2x3,
    #[casing(Lower)]
    DMat2x4,
    #[casing(Lower)]
    DMat3x2,
    #[casing(Lower)]
    DMat3x3,
    #[casing(Lower)]
    DMat3x4,
    #[casing(Lower)]
    DMat4x2,
    #[casing(Lower)]
    DMat4x3,
    #[casing(Lower)]
    DMat4x4,

    // Angle and trigonometry
    #[casing(Lower)]
    Radians,
    #[casing(Lower)]
    Degrees,
    #[casing(Lower)]
    Sin,
    #[casing(Lower)]
    Cos,
    #[casing(Lower)]
    Tan,
    #[casing(Lower)]
    Asin,
    #[casing(Lower)]
    Acos,
    #[casing(Lower)]
    Atan,
    #[casing(Lower)]
    Sinh,
    #[casing(Lower)]
    Cosh,
    #[casing(Lower)]
    Tanh,
    #[casing(Lower)]
    Asinh,
    #[casing(Lower)]
    Acosh,
    #[casing(Lower)]
    Atanh,
    #[casing(Lower)]
    Atan2,

    // Exponential
    #[casing(Lower)]
    Pow,
    #[casing(Lower)]
    Exp,
    #[casing(Lower)]
    Log,
    #[casing(Lower)]
    Exp2,
    #[casing(Lower)]
    Log2,
    #[casing(Lower)]
    Sqrt,
    #[casing(Lower)]
    Inversesqrt,

    // Common
    #[casing(Lower)]
    Abs,
    #[casing(Lower)]
    Sign,
    #[casing(Lower)]
    Floor,
    #[casing(Lower)]
    Trunc,
    #[casing(Lower)]
    Round,
    #[custom("roundEven")]
    RoundEven,
    #[casing(Lower)]
    Ceil,
    #[casing(Lower)]
    Fract,
    #[casing(Lower)]
    Mod,
    #[casing(Lower)]
    Modf,
    #[casing(Lower)]
    Min,
    #[casing(Lower)]
    Max,
    #[casing(Lower)]
    Clamp,
    #[casing(Lower)]
    Mix,
    #[casing(Lower)]
    Step,
    #[casing(Lower)]
    Smoothstep,
    #[casing(Lower)]
    Isnan,
    #[casing(Lower)]
    Isinf,
    #[custom("floatBitsToInt")]
    FloatBitsToInt,
    #[custom("floatBitsToUint")]
    FloatBitsToUint,
    #[custom("intBitsToFloat")]
    IntBitsToFloat,
    #[custom("uintBitsToFloat")]
    UintBitsToFloat,
    #[casing(Lower)]
    Fma,
    #[casing(Lower)]
    Frexp,
    #[casing(Lower)]
    Ldexp,

    // Pack / unpack
    #[custom("packUnorm2x16")]
    PackUnorm2x16,
    #[custom("packUnorm4x8")]
    PackUnorm4x8,
    #[custom("packSnorm2x16")]
    PackSnorm2x16,
    #[custom("packSnorm4x8")]
    PackSnorm4x8,
    #[custom("unpackUnorm2x16")]
    UnpackUnorm2x16,
    #[custom("unpackUnorm4x8")]
    UnpackUnorm4x8,
    #[custom("unpackSnorm2x16")]
    UnpackSnorm2x16,
    #[custom("unpackSnorm4x8")]
    UnpackSnorm4x8,
    #[custom("packHalf2x16")]
    PackHalf2x16,
    #[custom("unpackHalf2x16")]
    UnpackHalf2x16,
    #[custom("packDouble2x32")]
    PackDouble2x32,
    #[custom("unpackDouble2x32")]
    UnpackDouble2x32,

    // Geometric
    #[casing(Lower)]
    Length,
    #[casing(Lower)]
    Distance,
    #[casing(Lower)]
    Dot,
    #[casing(Lower)]
    Cross,
    #[casing(Lower)]
    Normalize,
    #[casing(Lower)]
    Faceforward,
    #[casing(Lower)]
    Reflect,
    #[casing(Lower)]
    Refract,

    // Matrix
    #[custom("matrixCompMult")]
    MatrixCompMult,
    #[custom("outerProduct")]
    OuterProduct,
    #[casing(Lower)]
    Transpose,
    #[casing(Lower)]
    Determinant,
    #[casing(Lower)]
    Inverse,

    // Vector relational
    #[custom("lessThan")]
    LessThan,
    #[custom("lessThanEqual")]
    LessThanEqual,
    #[custom("greaterThan")]
    GreaterThan,
    #[custom("greaterThanEqual")]
    GreaterThanEqual,
    #[casing(Lower)]
    Equal,
    #[custom("notEqual")]
    NotEqual,
    #[casing(Lower)]
    Any,
    #[casing(Lower)]
    All,
    #[casing(Lower)]
    Not,

    // Integer / bit manipulation
    #[custom("uaddCarry")]
    UaddCarry,
    #[custom("usubBorrow")]
    UsubBorrow,
    #[custom("umulExtended")]
    UmulExtended,
    #[custom("imulExtended")]
    ImulExtended,
    #[custom("bitfieldExtract")]
    BitfieldExtract,
    #[custom("bitfieldInsert")]
    BitfieldInsert,
    #[custom("bitfieldReverse")]
    BitfieldReverse,
    #[custom("bitCount")]
    BitCount,
    #[custom("findLSB")]
    FindLSB,
    #[custom("findMSB")]
    FindMSB,

    // Texture queries
    #[custom("textureSize")]
    TextureSize,
    #[custom("textureQueryLod")]
    TextureQueryLod,
    #[custom("textureQueryLevels")]
    TextureQueryLevels,
    #[custom("textureSamples")]
    TextureSamples,

    // Texture lookup
    #[casing(Lower)]
    Texture,
    #[custom("textureProj")]
    TextureProj,
    #[custom("textureLod")]
    TextureLod,
    #[custom("textureOffset")]
    TextureOffset,
    #[custom("texelFetch")]
    TexelFetch,
    #[custom("texelFetchOffset")]
    TexelFetchOffset,
    #[custom("textureProjOffset")]
    TextureProjOffset,
    #[custom("textureLodOffset")]
    TextureLodOffset,
    #[custom("textureProjLod")]
    TextureProjLod,
    #[custom("textureProjLodOffset")]
    TextureProjLodOffset,
    #[custom("textureGrad")]
    TextureGrad,
    #[custom("textureGradOffset")]
    TextureGradOffset,
    #[custom("textureProjGrad")]
    TextureProjGrad,
    #[custom("textureProjGradOffset")]
    TextureProjGradOffset,
    #[custom("textureGather")]
    TextureGather,
    #[custom("textureGatherOffset")]
    TextureGatherOffset,
    #[custom("textureGatherOffsets")]
    TextureGatherOffsets,

    // Compatibility-profile texture functions
    #[custom("texture1D")]
    Texture1D,
    #[custom("texture1DProj")]
    Texture1DProj,
    #[custom("texture1DLod")]
    Texture1DLod,
    #[custom("texture1DProjLod")]
    Texture1DProjLod,
    #[custom("texture2D")]
    Texture2D,
    #[custom("texture2DProj")]
    Texture2DProj,
    #[custom("texture2DLod")]
    Texture2DLod,
    #[custom("texture2DProjLod")]
    Texture2DProjLod,
    #[custom("texture3D")]
    Texture3D,
    #[custom("texture3DProj")]
    Texture3DProj,
    #[custom("texture3DLod")]
    Texture3DLod,
    #[custom("texture3DProjLod")]
    Texture3DProjLod,
    #[custom("textureCube")]
    TextureCube,
    #[custom("textureCubeLod")]
    TextureCubeLod,
    #[custom("shadow1D")]
    Shadow1D,
    #[custom("shadow2D")]
    Shadow2D,
    #[custom("shadow1DProj")]
    Shadow1DProj,
    #[custom("shadow2DProj")]
    Shadow2DProj,
    #[custom("shadow1DLod")]
    Shadow1DLod,
    #[custom("shadow2DLod")]
    Shadow2DLod,
    #[custom("shadow1DProjLod")]
    Shadow1DProjLod,
    #[custom("shadow2DProjLod")]
    Shadow2DProjLod,

    // Atomic counter
    #[custom("atomicCounterIncrement")]
    AtomicCounterIncrement,
    #[custom("atomicCounterDecrement")]
    AtomicCounterDecrement,
    #[custom("atomicCounter")]
    AtomicCounter,
    #[custom("atomicCounterAdd")]
    AtomicCounterAdd,
    #[custom("atomicCounterSubtract")]
    AtomicCounterSubtract,
    #[custom("atomicCounterMin")]
    AtomicCounterMin,
    #[custom("atomicCounterMax")]
    AtomicCounterMax,
    #[custom("atomicCounterAnd")]
    AtomicCounterAnd,
    #[custom("atomicCounterOr")]
    AtomicCounterOr,
    #[custom("atomicCounterXor")]
    AtomicCounterXor,
    #[custom("atomicCounterExchange")]
    AtomicCounterExchange,
    #[custom("atomicCounterCompSwap")]
    AtomicCounterCompSwap,

    // Atomic memory
    #[custom("atomicAdd")]
    AtomicAdd,
    #[custom("atomicMin")]
    AtomicMin,
    #[custom("atomicMax")]
    AtomicMax,
    #[custom("atomicAnd")]
    AtomicAnd,
    #[custom("atomicOr")]
    AtomicOr,
    #[custom("atomicXor")]
    AtomicXor,
    #[custom("atomicExchange")]
    AtomicExchange,
    #[custom("atomicCompSwap")]
    AtomicCompSwap,

    // Image
    #[custom("imageSize")]
    ImageSize,
    #[custom("imageSamples")]
    ImageSamples,
    #[custom("imageLoad")]
    ImageLoad,
    #[custom("imageStore")]
    ImageStore,
    #[custom("imageAtomicAdd")]
    ImageAtomicAdd,
    #[custom("imageAtomicMin")]
    ImageAtomicMin,
    #[custom("imageAtomicMax")]
    ImageAtomicMax,
    #[custom("imageAtomicAnd")]
    ImageAtomicAnd,
    #[custom("imageAtomicOr")]
    ImageAtomicOr,
    #[custom("imageAtomicXor")]
    ImageAtomicXor,
    #[custom("imageAtomicExchange")]
    ImageAtomicExchange,
    #[custom("imageAtomicCompSwap")]
    ImageAtomicCompSwap,

    // Geometry shader
    #[custom("EmitStreamVertex")]
    EmitStreamVertex,
    #[custom("EndStreamPrimitive")]
    EndStreamPrimitive,
    #[custom("EmitVertex")]
    EmitVertex,
    #[custom("EndPrimitive")]
    EndPrimitive,

    // Fragment derivatives
    #[custom("dFdx")]
    DFdx,
    #[custom("dFdy")]
    DFdy,
    #[custom("dFdxFine")]
    DFdxFine,
    #[custom("dFdyFine")]
    DFdyFine,
    #[custom("dFdxCoarse")]
    DFdxCoarse,
    #[custom("dFdyCoarse")]
    DFdyCoarse,
    #[casing(Lower)]
    Fwidth,
    #[custom("fwidthFine")]
    FwidthFine,
    #[custom("fwidthCoarse")]
    FwidthCoarse,

    // Fragment interpolation
    #[custom("interpolateAtCentroid")]
    InterpolateAtCentroid,
    #[custom("interpolateAtSample")]
    InterpolateAtSample,
    #[custom("interpolateAtOffset")]
    InterpolateAtOffset,

    // Noise (deprecated)
    #[casing(Lower)]
    Noise1,
    #[casing(Lower)]
    Noise2,
    #[casing(Lower)]
    Noise3,
    #[casing(Lower)]
    Noise4,

    // Shader invocation control
    #[casing(Lower)]
    Barrier,

    // Memory barriers
    #[casing(Lower)]
    MemoryBarrier,
    #[custom("memoryBarrierAtomicCounter")]
    MemoryBarrierAtomicCounter,
    #[custom("memoryBarrierBuffer")]
    MemoryBarrierBuffer,
    #[custom("memoryBarrierImage")]
    MemoryBarrierImage,
    #[custom("memoryBarrierShared")]
    MemoryBarrierShared,
    #[casing(Lower)]
    GroupMemoryBarrier,

    // Vulkan
    #[custom("subpassLoad")]
    SubpassLoad,
    #[custom("anyInvocation")]
    AnyInvocation,
    #[custom("allInvocations")]
    AllInvocations,
    #[custom("allInvocationsEqual")]
    AllInvocationsEqual,

    // Compatibility / legacy
    #[casing(Lower)]
    Ftransform,
}

impl core::fmt::Display for BuiltinFn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            // Scalar / vector constructors
            Self::Bool => "bool",
            Self::Int => "int",
            Self::Uint => "uint",
            Self::F32 => "f32",
            Self::F64 => "f64",

            Self::BVec2 => "bvec2",
            Self::BVec3 => "bvec3",
            Self::BVec4 => "bvec4",

            Self::IVec2 => "ivec2",
            Self::IVec3 => "ivec3",
            Self::IVec4 => "ivec4",

            Self::UVec2 => "uvec2",
            Self::UVec3 => "uvec3",
            Self::UVec4 => "uvec4",

            Self::Vec2 => "vec2",
            Self::Vec3 => "vec3",
            Self::Vec4 => "vec4",

            Self::DVec2 => "dvec2",
            Self::DVec3 => "dvec3",
            Self::DVec4 => "dvec4",

            // Matrix constructors
            Self::Mat2 => "mat2",
            Self::Mat3 => "mat3",
            Self::Mat4 => "mat4",
            Self::Mat2x2 => "mat2x2",
            Self::Mat2x3 => "mat2x3",
            Self::Mat2x4 => "mat2x4",
            Self::Mat3x2 => "mat3x2",
            Self::Mat3x3 => "mat3x3",
            Self::Mat3x4 => "mat3x4",
            Self::Mat4x2 => "mat4x2",
            Self::Mat4x3 => "mat4x3",
            Self::Mat4x4 => "mat4x4",

            Self::DMat2 => "dmat2",
            Self::DMat3 => "dmat3",
            Self::DMat4 => "dmat4",
            Self::DMat2x2 => "dmat2x2",
            Self::DMat2x3 => "dmat2x3",
            Self::DMat2x4 => "dmat2x4",
            Self::DMat3x2 => "dmat3x2",
            Self::DMat3x3 => "dmat3x3",
            Self::DMat3x4 => "dmat3x4",
            Self::DMat4x2 => "dmat4x2",
            Self::DMat4x3 => "dmat4x3",
            Self::DMat4x4 => "dmat4x4",

            // Angle and trigonometry
            Self::Radians => "radians",
            Self::Degrees => "degrees",
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Asin => "asin",
            Self::Acos => "acos",
            Self::Atan => "atan",
            Self::Sinh => "sinh",
            Self::Cosh => "cosh",
            Self::Tanh => "tanh",
            Self::Asinh => "asinh",
            Self::Acosh => "acosh",
            Self::Atanh => "atanh",
            Self::Atan2 => "atan2",

            // Exponential
            Self::Pow => "pow",
            Self::Exp => "exp",
            Self::Log => "log",
            Self::Exp2 => "exp2",
            Self::Log2 => "log2",
            Self::Sqrt => "sqrt",
            Self::Inversesqrt => "inversesqrt",

            // Common
            Self::Abs => "abs",
            Self::Sign => "sign",
            Self::Floor => "floor",
            Self::Trunc => "trunc",
            Self::Round => "round",
            Self::RoundEven => "roundEven",
            Self::Ceil => "ceil",
            Self::Fract => "fract",
            Self::Mod => "mod",
            Self::Modf => "modf",
            Self::Min => "min",
            Self::Max => "max",
            Self::Clamp => "clamp",
            Self::Mix => "mix",
            Self::Step => "step",
            Self::Smoothstep => "smoothstep",
            Self::Isnan => "isnan",
            Self::Isinf => "isinf",
            Self::FloatBitsToInt => "floatBitsToInt",
            Self::FloatBitsToUint => "floatBitsToUint",
            Self::IntBitsToFloat => "intBitsToFloat",
            Self::UintBitsToFloat => "uintBitsToFloat",
            Self::Fma => "fma",
            Self::Frexp => "frexp",
            Self::Ldexp => "ldexp",

            // Pack / unpack
            Self::PackUnorm2x16 => "packUnorm2x16",
            Self::PackUnorm4x8 => "packUnorm4x8",
            Self::PackSnorm2x16 => "packSnorm2x16",
            Self::PackSnorm4x8 => "packSnorm4x8",
            Self::UnpackUnorm2x16 => "unpackUnorm2x16",
            Self::UnpackUnorm4x8 => "unpackUnorm4x8",
            Self::UnpackSnorm2x16 => "unpackSnorm2x16",
            Self::UnpackSnorm4x8 => "unpackSnorm4x8",
            Self::PackHalf2x16 => "packHalf2x16",
            Self::UnpackHalf2x16 => "unpackHalf2x16",
            Self::PackDouble2x32 => "packDouble2x32",
            Self::UnpackDouble2x32 => "unpackDouble2x32",

            // Geometric
            Self::Length => "length",
            Self::Distance => "distance",
            Self::Dot => "dot",
            Self::Cross => "cross",
            Self::Normalize => "normalize",
            Self::Faceforward => "faceforward",
            Self::Reflect => "reflect",
            Self::Refract => "refract",

            // Matrix
            Self::MatrixCompMult => "matrixCompMult",
            Self::OuterProduct => "outerProduct",
            Self::Transpose => "transpose",
            Self::Determinant => "determinant",
            Self::Inverse => "inverse",

            // Vector relational
            Self::LessThan => "lessThan",
            Self::LessThanEqual => "lessThanEqual",
            Self::GreaterThan => "greaterThan",
            Self::GreaterThanEqual => "greaterThanEqual",
            Self::Equal => "equal",
            Self::NotEqual => "notEqual",
            Self::Any => "any",
            Self::All => "all",
            Self::Not => "not",

            // Integer / bit manipulation
            Self::UaddCarry => "uaddCarry",
            Self::UsubBorrow => "usubBorrow",
            Self::UmulExtended => "umulExtended",
            Self::ImulExtended => "imulExtended",
            Self::BitfieldExtract => "bitfieldExtract",
            Self::BitfieldInsert => "bitfieldInsert",
            Self::BitfieldReverse => "bitfieldReverse",
            Self::BitCount => "bitCount",
            Self::FindLSB => "findLSB",
            Self::FindMSB => "findMSB",

            // Texture queries
            Self::TextureSize => "textureSize",
            Self::TextureQueryLod => "textureQueryLod",
            Self::TextureQueryLevels => "textureQueryLevels",
            Self::TextureSamples => "textureSamples",

            // Texture lookup
            Self::Texture => "texture",
            Self::TextureProj => "textureProj",
            Self::TextureLod => "textureLod",
            Self::TextureOffset => "textureOffset",
            Self::TexelFetch => "texelFetch",
            Self::TexelFetchOffset => "texelFetchOffset",
            Self::TextureProjOffset => "textureProjOffset",
            Self::TextureLodOffset => "textureLodOffset",
            Self::TextureProjLod => "textureProjLod",
            Self::TextureProjLodOffset => "textureProjLodOffset",
            Self::TextureGrad => "textureGrad",
            Self::TextureGradOffset => "textureGradOffset",
            Self::TextureProjGrad => "textureProjGrad",
            Self::TextureProjGradOffset => "textureProjGradOffset",
            Self::TextureGather => "textureGather",
            Self::TextureGatherOffset => "textureGatherOffset",
            Self::TextureGatherOffsets => "textureGatherOffsets",

            // Compatibility-profile texture functions
            Self::Texture1D => "texture1D",
            Self::Texture1DProj => "texture1DProj",
            Self::Texture1DLod => "texture1DLod",
            Self::Texture1DProjLod => "texture1DProjLod",
            Self::Texture2D => "texture2D",
            Self::Texture2DProj => "texture2DProj",
            Self::Texture2DLod => "texture2DLod",
            Self::Texture2DProjLod => "texture2DProjLod",
            Self::Texture3D => "texture3D",
            Self::Texture3DProj => "texture3DProj",
            Self::Texture3DLod => "texture3DLod",
            Self::Texture3DProjLod => "texture3DProjLod",
            Self::TextureCube => "textureCube",
            Self::TextureCubeLod => "textureCubeLod",
            Self::Shadow1D => "shadow1D",
            Self::Shadow2D => "shadow2D",
            Self::Shadow1DProj => "shadow1DProj",
            Self::Shadow2DProj => "shadow2DProj",
            Self::Shadow1DLod => "shadow1DLod",
            Self::Shadow2DLod => "shadow2DLod",
            Self::Shadow1DProjLod => "shadow1DProjLod",
            Self::Shadow2DProjLod => "shadow2DProjLod",

            // Atomic counter
            Self::AtomicCounterIncrement => "atomicCounterIncrement",
            Self::AtomicCounterDecrement => "atomicCounterDecrement",
            Self::AtomicCounter => "atomicCounter",
            Self::AtomicCounterAdd => "atomicCounterAdd",
            Self::AtomicCounterSubtract => "atomicCounterSubtract",
            Self::AtomicCounterMin => "atomicCounterMin",
            Self::AtomicCounterMax => "atomicCounterMax",
            Self::AtomicCounterAnd => "atomicCounterAnd",
            Self::AtomicCounterOr => "atomicCounterOr",
            Self::AtomicCounterXor => "atomicCounterXor",
            Self::AtomicCounterExchange => "atomicCounterExchange",
            Self::AtomicCounterCompSwap => "atomicCounterCompSwap",

            // Atomic memory
            Self::AtomicAdd => "atomicAdd",
            Self::AtomicMin => "atomicMin",
            Self::AtomicMax => "atomicMax",
            Self::AtomicAnd => "atomicAnd",
            Self::AtomicOr => "atomicOr",
            Self::AtomicXor => "atomicXor",
            Self::AtomicExchange => "atomicExchange",
            Self::AtomicCompSwap => "atomicCompSwap",

            // Image
            Self::ImageSize => "imageSize",
            Self::ImageSamples => "imageSamples",
            Self::ImageLoad => "imageLoad",
            Self::ImageStore => "imageStore",
            Self::ImageAtomicAdd => "imageAtomicAdd",
            Self::ImageAtomicMin => "imageAtomicMin",
            Self::ImageAtomicMax => "imageAtomicMax",
            Self::ImageAtomicAnd => "imageAtomicAnd",
            Self::ImageAtomicOr => "imageAtomicOr",
            Self::ImageAtomicXor => "imageAtomicXor",
            Self::ImageAtomicExchange => "imageAtomicExchange",
            Self::ImageAtomicCompSwap => "imageAtomicCompSwap",

            // Geometry shader
            Self::EmitStreamVertex => "EmitStreamVertex",
            Self::EndStreamPrimitive => "EndStreamPrimitive",
            Self::EmitVertex => "EmitVertex",
            Self::EndPrimitive => "EndPrimitive",

            // Fragment derivatives
            Self::DFdx => "dFdx",
            Self::DFdy => "dFdy",
            Self::DFdxFine => "dFdxFine",
            Self::DFdyFine => "dFdyFine",
            Self::DFdxCoarse => "dFdxCoarse",
            Self::DFdyCoarse => "dFdyCoarse",
            Self::Fwidth => "fwidth",
            Self::FwidthFine => "fwidthFine",
            Self::FwidthCoarse => "fwidthCoarse",

            // Fragment interpolation
            Self::InterpolateAtCentroid => "interpolateAtCentroid",
            Self::InterpolateAtSample => "interpolateAtSample",
            Self::InterpolateAtOffset => "interpolateAtOffset",

            // Noise
            Self::Noise1 => "noise1",
            Self::Noise2 => "noise2",
            Self::Noise3 => "noise3",
            Self::Noise4 => "noise4",

            // Shader invocation control
            Self::Barrier => "barrier",

            // Memory barriers
            Self::MemoryBarrier => "memoryBarrier",
            Self::MemoryBarrierAtomicCounter => "memoryBarrierAtomicCounter",
            Self::MemoryBarrierBuffer => "memoryBarrierBuffer",
            Self::MemoryBarrierImage => "memoryBarrierImage",
            Self::MemoryBarrierShared => "memoryBarrierShared",
            Self::GroupMemoryBarrier => "groupMemoryBarrier",

            // Vulkan
            Self::SubpassLoad => "subpassLoad",
            Self::AnyInvocation => "anyInvocation",
            Self::AllInvocations => "allInvocations",
            Self::AllInvocationsEqual => "allInvocationsEqual",

            // Compatibility / legacy
            Self::Ftransform => "ftransform",
        };

        f.write_str(s)
    }
}

pub enum BuiltinFnError {
    NotConstable,
    NotImplemented,
    IllegalParams,
    IllegalParamCount,
}

impl BuiltinFn {
    pub fn apply(&self, params: Vec<Literal>) -> Result<Literal, BuiltinFnError> {
        macro_rules! constructor {
            {
                $(
                    $argc:literal($($arg:literal),*) => {
                        $(
                            ($($enum_variant:ident($name:ident)),*) => $output:ident($($init:expr),*)
                        ),*$(,)?
                    }
                )*
            } => {
                match params.len() {
                    $(
                        $argc => {
                            match ($(params[$arg].clone()),*) {
                                $(
                                    #[allow(unused_parens)]
                                    ($(Literal::$enum_variant($name)),*) => Ok(Literal::$output([$($init),*]))
                                ),*,
                                _ => Err(BuiltinFnError::IllegalParams),
                            }
                        }
                    )*
                    _ => Err(BuiltinFnError::IllegalParamCount),
                }
            };
        }

        macro_rules! constructor_t4 {
            ($t1:ident, $t2:ident, $t3:ident, $t4:ident) => {
                constructor! {
                    1(0) => {
                        ($t4(a)) => $t4(a[0], a[1], a[2], a[3]),
                        ($t1(a)) => $t4(a, a, a, a),
                    }
                    2(0, 1) => {
                        ($t3(a), $t1(b)) => $t4(a[0], a[1], a[2], b),
                        ($t2(a), $t2(b)) => $t4(a[0], a[1], b[0], b[1]),
                        ($t1(a), $t3(b)) => $t4(a, b[0], b[1], b[2]),
                    }
                    3(0, 1, 2) => {
                        ($t2(a), $t1(b), $t1(c)) => $t4(a[0], a[1], b, c),
                        ($t1(a), $t2(b), $t1(c)) => $t4(a, b[0], b[1], c),
                        ($t1(a), $t1(b), $t2(c)) => $t4(a, b, c[0], c[1]),
                    }
                    4(0, 1, 2, 3) => {
                        ($t1(a), $t1(b), $t1(c), $t1(d)) => $t4(a, b, c, d),
                    }
                }
            };
        }

        macro_rules! constructor_t3 {
            ($t1:ident, $t2:ident, $t3:ident) => {
                constructor! {
                    1(0) => {
                        ($t3(a)) => $t3(a[0], a[1], a[2]),
                        ($t1(a)) => $t3(a, a, a),
                    }
                    2(0, 1) => {
                        ($t2(a), $t1(b)) => $t3(a[0], a[1], b),
                        ($t1(a), $t2(b)) => $t3(a, b[0], b[1]),
                    }
                    3(0, 1, 2) => {
                        ($t1(a), $t1(b), $t1(c)) => $t3(a, b, c),
                    }
                }
            };
        }

        macro_rules! constructor_t2 {
            ($t1:ident, $t2:ident) => {
                constructor! {
                    1(0) => {
                        ($t2(a)) => $t2(a[0], a[1]),
                        ($t1(a)) => $t2(a, a),
                    }
                    2(0, 1) => {
                        ($t1(a), $t1(b)) => $t2(a, b),
                    }
                }
            };
        }

        macro_rules! constructor_t1 {
            ($t1:ident) => {
                match params.len() {
                    1 => {
                        match params[0] {
                            Literal::$t1(a) => Ok(Literal::$t1(a)),
                            _ => Err(BuiltinFnError::IllegalParams),
                        }
                    }
                    _ => Err(BuiltinFnError::IllegalParamCount),
                }
            }
        }

        macro_rules! per_field_fn {
            (
                ($($t1:ident),*),
                ($($t2:ident),*),
                ($($t3:ident),*),
                ($($t4:ident),*),
                $name:ident => $expr:expr
            ) => {
                if params.len() != 1 {
                    return Err(BuiltinFnError::IllegalParamCount);
                }
                else {
                    match params[0] {
                        $(
                            Literal::$t1($name) => Ok(Literal::$t1($expr))
                        ),*,
                        $(
                            Literal::$t2(a) => Ok(Literal::$t2(a.map(|$name| $expr)))
                        ),*,
                        $(
                            Literal::$t3(a) => Ok(Literal::$t3(a.map(|$name| $expr)))
                        ),*,
                        $(
                            Literal::$t4(a) => Ok(Literal::$t4(a.map(|$name| $expr)))
                        ),*,
                        _ => Err(BuiltinFnError::IllegalParams),
                    }
                }
            };
        }

        macro_rules! per_field_fn2 {
            (
                ($($t1:ident),*),
                ($($t2:ident),*),
                ($($t3:ident),*),
                ($($t4:ident),*),
                ($name1:ident, $name2:ident) => $expr:expr
            ) => {
                if params.len() != 2 {
                    return Err(BuiltinFnError::IllegalParamCount);
                }
                else {
                    match (params[0].clone(), params[1].clone()) {
                        $(
                        (Literal::$t1($name1), Literal::$t1($name2)) => Ok(Literal::$t1($expr))
                        ),*,
                        $(
                        (Literal::$t2(a), Literal::$t2(b)) => Ok(Literal::$t2(a.into_iter().zip(b.into_iter()).map(|($name1, $name2)| $expr).collect_array::<2>().unwrap()))
                        ),*,
                        $(
                        (Literal::$t3(a), Literal::$t3(b)) => Ok(Literal::$t3(a.into_iter().zip(b.into_iter()).map(|($name1, $name2)| $expr).collect_array::<3>().unwrap()))
                        ),*,
                        $(
                        (Literal::$t4(a), Literal::$t4(b)) => Ok(Literal::$t4(a.into_iter().zip(b.into_iter()).map(|($name1, $name2)| $expr).collect_array::<4>().unwrap()))
                        ),*,
                        _ => Err(BuiltinFnError::IllegalParams),
                    }
                }
            };
        }

        macro_rules! per_field_fn_floatlike {
            ($name:ident => $expr:expr) => {
                per_field_fn!((FloatLit, DoubleLit), (Vec2, DVec2), (Vec3, DVec3), (Vec4, DVec4), $name => $expr)
            };
        }

        macro_rules! per_field_fn2_floatlike {
            (($name1:ident, $name2:ident) => $expr:expr) => {
                per_field_fn2!((FloatLit, DoubleLit), (Vec2, DVec2), (Vec3, DVec3), (Vec4, DVec4), ($name1, $name2) => $expr)
            };
        }

        match self {
            BuiltinFn::F32 => constructor_t1!(FloatLit),
            BuiltinFn::Int => constructor_t1!(IntLit),
            BuiltinFn::Bool => constructor_t1!(BoolLit),
            BuiltinFn::F64 => constructor_t1!(DoubleLit),
            BuiltinFn::Uint => constructor_t1!(UIntLit),



            BuiltinFn::Vec2 => constructor_t2!(FloatLit, Vec2),
            BuiltinFn::IVec2 => constructor_t2!(IntLit, IVec2),
            BuiltinFn::BVec2 => constructor_t2!(BoolLit, BVec2),
            BuiltinFn::DVec2 => constructor_t2!(DoubleLit, DVec2),
            BuiltinFn::UVec2 => constructor_t2!(UIntLit, UVec2),

            BuiltinFn::Mat2 => constructor_t2!(Vec2, Mat2),
            BuiltinFn::Mat3x2 => constructor_t2!(Vec3, Mat3x2),
            BuiltinFn::Mat4x2 => constructor_t2!(Vec4, Mat4x2),

            BuiltinFn::DMat2 => constructor_t2!(DVec2, DMat2),
            BuiltinFn::DMat3x2 => constructor_t2!(DVec3, DMat3x2),
            BuiltinFn::DMat4x2 => constructor_t2!(DVec4, DMat4x2),



            BuiltinFn::Vec3 => constructor_t3!(FloatLit, Vec2, Vec3),
            BuiltinFn::IVec3 => constructor_t3!(IntLit, IVec2, IVec3),
            BuiltinFn::BVec3 => constructor_t3!(BoolLit, BVec2, BVec3),
            BuiltinFn::DVec3 => constructor_t3!(DoubleLit, DVec2, DVec3),
            BuiltinFn::UVec3 => constructor_t3!(UIntLit, UVec2, UVec3),

            BuiltinFn::Mat2x3 => constructor_t3!(Vec2, Mat2, Mat2x3),
            BuiltinFn::Mat3 => constructor_t3!(Vec3, Mat3x2, Mat3),
            BuiltinFn::Mat4x3 => constructor_t3!(Vec4, Mat4x2, Mat4x3),

            BuiltinFn::DMat2x3 => constructor_t3!(DVec2, DMat2, DMat2x3),
            BuiltinFn::DMat3 => constructor_t3!(DVec3, DMat3x2, DMat3),
            BuiltinFn::DMat4x3 => constructor_t3!(DVec4, DMat4x2, DMat4x3),



            BuiltinFn::Vec4 => constructor_t4!(FloatLit, Vec2, Vec3, Vec4),
            BuiltinFn::IVec4 => constructor_t4!(IntLit, IVec2, IVec3, IVec4),
            BuiltinFn::BVec4 => constructor_t4!(BoolLit, BVec2, BVec3, BVec4),
            BuiltinFn::DVec4 => constructor_t4!(DoubleLit, DVec2, DVec3, DVec4),
            BuiltinFn::UVec4 => constructor_t4!(UIntLit, UVec2, UVec3, UVec4),

            BuiltinFn::Mat2x4 => constructor_t4!(Vec2, Mat2, Mat2x3, Mat2x4),
            BuiltinFn::Mat3x4 => constructor_t4!(Vec3, Mat3x2, Mat3, Mat3x4),
            BuiltinFn::Mat4 => constructor_t4!(Vec4, Mat4x2, Mat4x3, Mat4),

            BuiltinFn::DMat2x4 => constructor_t4!(DVec2, DMat2, DMat2x3, DMat2x4),
            BuiltinFn::DMat3x4 => constructor_t4!(DVec3, DMat3x2, DMat3, DMat3x4),
            BuiltinFn::DMat4 => constructor_t4!(DVec4, DMat4x2, DMat4x3, DMat4),

            BuiltinFn::Radians => per_field_fn_floatlike!(f => f.to_radians()),
            BuiltinFn::Degrees => per_field_fn_floatlike!(f => f.to_degrees()),
            BuiltinFn::Sin => per_field_fn_floatlike!(f => f.sin()),
            BuiltinFn::Cos => per_field_fn_floatlike!(f => f.cos()),
            BuiltinFn::Tan => per_field_fn_floatlike!(f => f.tan()),
            BuiltinFn::Asin => per_field_fn_floatlike!(f => f.asin()),
            BuiltinFn::Acos => per_field_fn_floatlike!(f => f.acos()),
            BuiltinFn::Atan => per_field_fn_floatlike!(f => f.atan()),
            BuiltinFn::Sinh => per_field_fn_floatlike!(f => f.sinh()),
            BuiltinFn::Cosh => per_field_fn_floatlike!(f => f.cosh()),
            BuiltinFn::Tanh => per_field_fn_floatlike!(f => f.tanh()),
            BuiltinFn::Asinh => per_field_fn_floatlike!(f => f.asinh()),
            BuiltinFn::Acosh => per_field_fn_floatlike!(f => f.acosh()),
            BuiltinFn::Atanh => per_field_fn_floatlike!(f => f.atanh()),
            BuiltinFn::Exp => per_field_fn_floatlike!(f => f.exp()),
            BuiltinFn::Log => per_field_fn_floatlike!(f => f.log10()),
            BuiltinFn::Exp2 => per_field_fn_floatlike!(f => f.exp2()),
            BuiltinFn::Log2 => per_field_fn_floatlike!(f => f.log2()),
            BuiltinFn::Sqrt => per_field_fn_floatlike!(f => f.sqrt()),
            BuiltinFn::Inversesqrt => per_field_fn_floatlike!(f => 1.0 / f.sqrt()),

            BuiltinFn::Pow => per_field_fn2_floatlike!((a, b) => a.powf(b)),
            BuiltinFn::Atan2 => per_field_fn2_floatlike!((a, b) => a.atan2(b)),



            _ => Err(BuiltinFnError::NotConstable),
        }
    }
}
