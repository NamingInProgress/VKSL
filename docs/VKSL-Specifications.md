# **VKSL Language Specification**
### **Version 0.1**

## Abstract
VKSL (VKE Shader Language) is a statically typed GLSL-style language designed for writing GPU programs targeting 
the SPIR-V intermediate representation. The language aims to simplify shader development by removing unnecessary 
explicitness while retaining control over program behavior and resource representation. 
This document specifies the syntax and semantics of VKSL and the mapping of its constructs to SPIR-V

## 1. Introduction
### 1.1 Goals
The primary goal of VKSL is to simplify the process of creating GPU programs by removing much of the explicitness 
required by other shader languages while retaining complete control over GPU resources. VKSL additionally allows 
the definition of multiple shader stages within a single source file via the stage specific functions specified 
in Section 7.3.2

### 1.2 Non-goals
VKSL does not aim to abstract away the underlying GPU execution model. The programmer retains explicit control over 
GPU resources, shader stages, and other aspects of GPU program execution where such control is necessary.

VKSL does not aim to provide complete compatibility with GLSL. While VKSL uses a GLSL-style syntax, 
its syntax and semantics are independently defined by this specification.

### 1.3 Conformance
An implementation conforms to this specification, if it correctly implements the syntax and semantics 
defined in this document and produces SPIR-V conforming to the requirements specified by VKSL

An implementation may provide extensions to VKSL, provided that such extensions do not alter the behavior of valid VKSL programs.

## 2. Terminology and Conventions
### 2.1 Terminology
**Implementation** - A compiler or other software that is capable of processing VKSL source code.
**Shader stage** - A programmable stage of the GPU pipeline.

### 2.2 Conventions
The terms **shall**, **may**, and **should** are used to indicate requirements, permitted behavior, and recommendations respectively.

VKSL source code is represented using monospaced formatting.

Unless otherwise specified, identifiers are case-sensitive.

## 3. Lexical Structure
### 3.1 Character Set
VKSL source files shall be encoded using UTF-8.

Characters outside the ASCII character set shall not occur outside comments and string literals.

### 3.2 Whitespace and Comments
VKSL whitespace shall consist of either the space character (U+0020) or the horizontal tab character (U+0009). 
Other whitespace characters are not permitted and shall result in a compilation error.

Line terminators are permitted between tokens and terminate single-line comments.

VKSL defines two types of comments:
 - **Single line comments**, beginning with `//` and continuing until the end of the line 
 - **Block comments**, beginning with `/*` and ending with `*/`

### 3.3 Identifiers
An identifier is a sequence of letters, digits and underscores, beginning with a letter or underscore.

Identifiers are **case-sensitive**. For example `value`, `Value` and `VALUE` are separate identifiers.

Identifiers shall not be identical to reserved keywords defined in Section 3.4
Identifiers shall not begin with the prefix `VKSL_`. This prefix is reserved for VKSL built-in variables defined in Section 3.5

### 3.4 Keywords
VKSL reserves a set of keywords for use by the language. A keyword has a predefined meaning, and shall not be used 
as an identifier. 

The following keywords are reserved:

| Keyword        | Purpose                                                                     |
|----------------|-----------------------------------------------------------------------------|
| fn             | Declares a function                                                         |
| struct         | Declares a structure                                                        |
| this           | The instance on which a structure method was call on (see Segment 4.5.2     | 
| if             | Conditional execution                                                       |
| else           | Defines the alternative branch of a conditional statement                   |
| while          | Defines a while loop                                                        |
| for            | Defines a for loop                                                          |
| return         | Returns from a function                                                     |
| yield          | Returns a value from an annonymous block statement                          |
| let            | Declares a mutable variable                                                 |
| const          | Declares an immutable variable                                              |
| include        | Includes a different module                                                 |
| extension      | Defines the usage of an extension                                           |
| enable         | Enables an extension                                                        |
| require        | Specifies an extension is required                                          |
| warn           | If this extension is used warnings will be emitted                          |
| disable        | Disables an extension                                                       |
| input          | Declares a vertex input variable                                            |
| flat           | Specifies an input is interpolated using `flat`                             |
| smooth         | Specifies an input is interpolated using `smooth`                           |
| noperpspecitve | Specifies an input is interpolated using `noperspective`                    |
| output         | Declares a fragment output variable                                         |
| provide        | Declares an input to the fragment shader, that is not defined using `input` |
| push_constants | Declares a push constants block                                             |
| uniform        | Declares a uniform                                                          |
| std430         | Marks a buffer resource as using the STD430 layout                          |
| std140         | Marks a buffer resource as using the STD140 layout                          |
| buffer         | Declares a GPU buffer                                                       |
| readonly       | Marks a buffer declared with `buffer` as read-only                          |
| writeonly      | Marks a buffer declared with `buffer` as write-only                         |
| break          | Exits the current loop                                                      |
| continue       | Skips the current iteration in a loop                                       |

### 3.5 Built-in Variables
VKSL defines a set of built-in variables that provide access to functionality or resources supplied by the shader
environment.

Built-in identifiers beginning with `VKSL_` are reserved and shall not be redeclared by a VKSL program.

The following built-in identifiers are defined:

| Identifier | Purpose                                                  |
|------------|----------------------------------------------------------|
| VKSL_pos   | Specifies the output position of the vertex shader stage |

### 3.6 Literals

A literal is a syntactic representation of a constant value.

#### 3.6.1 Boolean Literals
Boolean literals represent values of a boolean type.

VKSL provides only two literals representing a boolean, `true` and `false`

#### 3.6.2 Integer Literals

Integer literals represent values of an integer type.

##### 3.6.2.1 Signed Integer Literals
Any integer literal without an unsigned suffix has type `int`

Examples:
```
42
0
0xFF
0b1100
```

##### 3.6.2.2 Unsigned Integer Literals
An integer literal with the suffix `u` or `U` has type `uint`.

Examples:
```
0U
54u
0xFFu
0b1100U
```

#### 3.6.3 Floating-Point Literals
Floating-point literals represent values of a floating-point type. 

##### 3.6.3.1 Single-Precision Floating-Point Literals
A floating-point literal without the `f64` suffix has type `f32`. The optional suffix `f32` may be used to 
explicitly specify the type.

Examples:
```
42.05
1.5e-3
123f32
```

##### 3.6.3.2 Double-Precision Floating-Point Literals
A floating-point literal with a double-precision suffix `f64` has type `f64`.

Examples:
```
4.993f64
1.5e-5f64
2.34f64
```

#### 3.6.4 Lexical Syntax of Literals
```
digit ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
digits ::= digit+

hexadecimal_digit ::= digit | "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F"
hexadecimal_digits ::= hexadecimal_digit+

binary_digit ::= "0" | "1"
binary_digits ::= binary_digit+
```
##### 3.6.4.1 Booleans
A boolean literal can be one of two keywords. 
- `false` represents the boolean value `false`
- `true` represents the boolean value `true`

Definition:
```
boolean_literal ::= "true" | "false"
```

##### 3.6.4.2 Integers
An integer literal may be written in decimal, hexadecimal or binary notation.

Decimal integers consist of one or more decimal digits. Hexadecimal integer literals begin with the prefix `0x` followed
by one or more hexadecimal digits. Binary integer literals begin with the prefix `0b` followed by one or more binary digits.

These literals may additionally be followed with a `u` or `U` suffix to specify that the literal is of the `uint` type.

Definition:
```
integer_literal ::= decimal_integer | hexadecimal_integer | binary_integer

decimal_integer ::= digits integer_suffix?
hexadecimal_integer ::= "0x" hexadecimal_digits integer_suffix?
binary_integer ::= "0b" binary_digits integer_suffix?

integer_suffix ::= "u" | "U"
```

##### 3.6.4.3 Floating-Point Literals
A floating-point literal may only be written in decimal notation.

A floating-point literal consists of a decimal integer part followed by a decimal point and an optional fractional part, 
or a decimal integer part followed by an exponent. A floating-point literal may additionally have a precision suffix.

A floating-point literal may use scientific notation.
The exponent is introduced by `e` or `E` and may optionally be preceded by `+` or `-`.

If a floating-point literal is followed by a `f32` suffix, the type is explicitly set to `f32`. If the suffix is `f64` the type 
is explicitly set to `f64`

A floating-point literal without an f64 suffix has type f32.

Definition:
```
floating_point_literal ::= digits "." digits? exponent? floating_point_suffix?
                           | digits exponent floating_point_suffix?
                           | digits floating_point_suffix
floating_point_suffix ::= "f32" | "f64"
exponent ::= ("e" | "E") ("+" | "-")? digits
```

## 4. Types
### 4.1 Scalar Types
A scalar type represents a single value. VKSL provides boolean, integer and floating-point types.

| Type | Description                                    |
|------|------------------------------------------------|
| bool | Boolean value                                  |
| int  | Signed 32-bit integer                          |
| uint | Unsigned 32-bit integer                        |
| f32  | 32-bit (single-precision) floating-point value |
| f64  | 64-bit (double-precision) floating-point value |

#### 4.1.1 Bool
`bool` represents a boolean value. A `bool` has exactly two possible values: `true` or `false`

#### 4.1.2 Int
`int` represents a 32-bit signed integer type. Its range is from `-2^31` to `2^31-1`

#### 4.1.3 UInt
`uint` represents a 32-bit unsigned integer type. Its range is from `0` to `2^32-1`

#### 4.1.4 F32
`f32` represents a 32-bit single-precision floating-point type.

#### 4.1.5 F64
`f64` represents a 64-bit double-precision floating-point type.

### 4.2 Vector Types
A vector type represents an ordered collection of two, three or four values of the same scalar type.
Vectors can only be 2, 3 or 4 dimensional.

The name of a vector type consists of a scalar-type prefix (e.g. `b`, `i`, `u`, `d`), followed by `vec` and 
the number of elements `N` ranging from two to four.

VKSL provides vector types for each scalar type:

| Scalar | Vector Types              |
|--------|---------------------------|
| `bool` | `bvec2`, `bvec3`, `bvec4` |
| `int`  | `ivec2`, `ivec3`, `ivec4` |
| `uint` | `uvec2`, `uvec3`, `uvec4` |
| `f32`  | `vec2`, `vec3`, `vec4`    |
| `f64`  | `dvec2`, `dvec3`, `dvec4` |

#### 4.2.1 Vector Component Access
To access specific components of a vector you may use the following expressions:

| Expression | Component                          |
|------------|------------------------------------|
| vector.x   | The first component of the vector  |
| vector.y   | The second component of the vector |
| vector.z   | The third component of the vector  |
| vector.w   | The fourth component of the vector |

Additionally, `x`, `y`, `z` and `w` may be replaced with `r`, `g`, `b`, `a` respectively.

The first and second components may additionally be accessed using the aliases `u` and `v`, respectively.

#### 4.2.2 Swizzles
A vector swizzle is an expression consisting of a vector followed by a sequence of two or more component selectors. 
A swizzle may contain between one and four component selectors.

A swizzle shall use component selectors from a single naming set. 
The `xyzw`, `rgba`, and `uv` naming sets shall not be mixed within a single swizzle.

A component selector shall refer to a component that exists in the vector. For example, `vec2.z` is invalid.

A swizzle expression has the same scalar type as the vector from which it is derived and a component count equal to the number of selectors.

Examples:
```
let v = vec4(1.0, 2.0, 3.0, 4.0);

v.xyz; // vec3(1.0, 2.0, 3.0)
v.zyx; // vec3(3.0, 2.0, 1.0)
v.xxxx; // vec4(1.0, 1.0, 1.0, 1.0)
v.rg; // vec2(1.0, 2.0)
```

A swizzle expression may be used as the destination of an assignment provided that no component occurs more than once 
within the swizzle.
```
v.xy = vec2(5.0, 6.0);
v.zyx = vec3(3.0, 2.0, 1.0);

// A swizzle containing duplicate components shall not be assignable.

v.xx = vec2(1.0, 2.0); // invalid
v.xxy = vec3(1.0, 2.0, 3.0); // invalid
```

### 4.3 Matrix Types
A matrix type represents an ordered collection of floating-point vectors.
Matrices are composed of columns, with each column containing the number of
elements specified by the number of rows.

VKSL provides matrix types with two through four columns and two through four
rows. All matrix types contain `f32` or `f64` components.

Matrices are column-major. The first index of a matrix selects a column, and
the second index selects an element within that column.

For example, `mat4` represents a matrix with four columns and four rows of type `f32`:
```
let matrix = mat4(1);

matrix[0]       // first column
matrix[0][0]    // first element of the first column
matrix[2][1]    // second element of the third column
```

#### 4.3.1 Single-Precision Matrices
Single-precision matrices contain components of type `f32`:

A matrix type is defined as `matCxR`, with `C` columns and `R` rows ranging from 2 to 4.

For a matrix with the same amount of rows as columns the type is defined as `matN` with N being the number of rows and columns ranging from 2 to 4.

| Type   | Columns | Rows |
|--------|---------|------|
| mat2   | 2       | 2    |
| mat3   | 3       | 3    |
| mat4   | 4       | 4    |
| mat2x3 | 2       | 3    |
| mat2x4 | 2       | 4    |
| mat3x2 | 3       | 2    |
| mat3x4 | 3       | 4    |
| mat4x2 | 4       | 2    |
| mat4x3 | 4       | 3    |

#### 4.3.2 Double-Precision Matrices
Double-precision matrices contain components of type `f64`:

The definition of the matrix type is the same as for single-precision matrices defined in Section 4.3.1, but use the `dmat` prefix instead of the `mat` prefix.

| Type    | Columns | Rows |
|---------|---------|------|
| dmat2   | 2       | 2    |
| dmat3   | 3       | 3    |
| dmat4   | 4       | 4    |
| dmat2x3 | 2       | 3    |
| dmat2x4 | 2       | 4    |
| dmat3x2 | 3       | 2    |
| dmat3x4 | 3       | 4    |
| dmat4x2 | 4       | 2    |
| dmat4x3 | 4       | 3    |

### 4.4 Arrays
An array represents an ordered collection of elements of the same type.
All elements of the array shall have the same type.

An array type is specified by an element type, optionally followed by an integer literal, enclosed in square brackets.

#### 4.4.1 Runtime Size Arrays
A runtime size array is created when a size is not given at compile time.

Runtime size arrays are only permitted in Shader Storage Buffer Objects.

#### 4.4.2 Examples & Definition
Examples:
```
int[123]
vec3[10]
vec4[] // Arrays without a predefined size shall only appear in Shader Storage Buffer Objects as defined in Section 9.2.1
```

Definition:
```
array_type ::= type "[" digits? "]"
```

The number of elements in an array is called its length.

An array element is accessed using the indexing operation (`arr[index]`). The index shall evaluate to a non-negative integer value.

Array indices are zero-indexed. For an array of length `N`, valid indices range from `0` to `N-1`.

### 4.5 Structures
A structure represents a collection of a number of named members. Each member has a type and an identifier. Members of a struct may have different types.

A structure is declared using the `struct` keyword.

The identifier following the `struct` keyword introduces a structure type with the specified name.

A structure type may be constructed by invoking its type name as a constructor. Constructor arguments correspond to the structure's members in declaration order.

Example:
```
struct Camera {
    projection, view: mat4;
    position, lookDir: vec3;
}
```

Each member of the struct is accessed using the member access operator `.`.

```
let camera = Camera(projection, view, position, lookDir);
let cameraDirection = camera.lookDir; 
```

A structure member shall have a type that is valid in the context in which
the structure is declared.

#### 4.5.1 Structure Declaration
The syntax of a structure declaration is:
```
struct_decl ::= "struct" identifier "{" struct_member* "}"
struct_member ::= (identifier_list ":" type ";") | method_declaration
identifier_list ::= identifier ("," identifier)*
```

Each member shall have a unique identifier within the structure.

A structure shall not directly or indirectly contain itself as a member.

A structure may contain other structures.

A structure shall not be empty. To be valid, a structure requires one or more members.

#### 4.5.2 Methods in Structures
A structure may contain method declarations. A method is a function associated with a structure type and may access 
the members of the structure directly by their identifiers or via the `this` keyword.

Method declaration follows the same convention as specified in Section 7.3

If the instance is not declared as `const`, members of the instance may be modified directly or via `this`.

If the instance is declared as `const`, modification of its members either directly or via `this` is not permitted.

The `this` expression may be passed as an argument to functions and methods.

Example:
```
struct MyCustomStructure {
    a, b: int;
    
    fn sum() -> int {
        return this.a + this.b;
    }
}

fn main() {
    let s = MyCustomStructure(5, 10);
   
    let sum = s.sum();
}
```

### 4.6 Tuple Type
A tuple is an ordered collection of a fixed number of values, where each element may have a different type.

The type of a tuple is determined by the types and order of its elements. For example, `(vec3, mat4)` and `(mat4, vec3)` are distinct types.

A tuple type is specified by a comma-separated list of types enclosed in
parentheses.

Definition:
```text
tuple_type ::= "(" type_list ")"
type_list ::= type ("," type)*
```

A tuple value is constructed using a comma-separated list of expressions enclosed in parentheses.

A tuple element is accessed using the indexing operation. The index shall be
a non-negative integer constant less than the number of elements in the tuple.

Example:
```
let position = ...;
let projection_matrix = ...;
let pos_proj: (vec3, mat4) = (position, projection_matrix);

position = pos_proj[0];
pos_proj[0] = position
```

### 4.7 Resource Types
Resource types represent resources that are provided to a shader by the graphics API. Unlike ordinary value types, resource types represent access
to externally provided GPU resources.

VKSL provides resource types for sampled images, storage images, and samplers.

Resource types are opaque and cannot be constructed or modified as ordinary values.

If no packing type is specified, std140 shall be used for uniform buffers and std430 shall be used for shader storage buffers.

#### 4.7.1 Sampled Images
The `image1D`, `image2D`, `image3D`, and `imageCube` types represent image resources.

The following sampled image types are provided:

| Type        | Description                     |
|-------------|---------------------------------|
| `image1D`   | One-dimensional sampled image   |
| `image2D`   | Two-dimensional sampled image   |
| `image3D`   | Three-dimensional sampled image |
| `imageCube` | Cube sampled image              |

#### 4.7.2 Combined Image Samplers
A combined image sampler type represents a sampled image together with a sampler.

The following combined image sampler types are provided:

| Type          | Description                              |
|---------------|------------------------------------------|
| `sampler1D`   | One-dimensional combined image sampler   |
| `sampler2D`   | Two-dimensional combined image sampler   |
| `sampler3D`   | Three-dimensional combined image sampler |
| `samplerCube` | Cube combined image sampler              |

### 4.8 Resource Declarations
A resource declaration declares a resource that is provided to a shader by the graphics API.

The `uniform` keyword indicates that the resource is provided by the shader environment rather than created by the shader.

A resource declaration specifies a descriptor set, binding, resource type, and identifier.

The `set` and `binding` values specify the descriptor set and binding to which the resource is assigned.

A buffer declaration may contain an inline structure declaration. The identifier preceding the structure body 
specifies the name of the structure type, while the identifier following the structure body specifies the name of the resource instance.

Definition:
```
uniform_decl ::= ("uniform" descriptor_location identifier: type_specifier ";") | ubo_decl | ssbo_decl
descriptor_location ::= "set" "=" integer_literal "binding" "=" integer_literal
packing_type ::= ("std140" | "std430")
```

#### 4.8.1 Uniform Buffers
A uniform buffer provides read-only access to a block of uniformly laid-out data.

The struct layout of a uniform buffer block is subject to memory alignment rules.

Definition:
```
ubo_decl ::= "uniform" descriptor_location packing_type? identifier: type_specifier ";"
```

Example:
```
uniform set = 0 binding = 0 Camera {
    projection, view: mat4;
    position: vec3;
} camera;
```

The declaration above defines a structure type named Camera and a uniform
buffer resource named camera.

The structure type is equivalent to:
```
struct Camera {
    projection, view: mat4;
    position: vec3;
}
```
and the resource is equivalent to:
```
uniform set = 0 binding = 0 Camera camera;
```

#### 4.8.2 Shader Storage Buffers
A shader storage buffer provides access to a block of GPU data.

The struct layout of a shader storage buffer block is subject to memory alignment rules.

A shader storage buffer may be defined with an access modifier.

VKSL defines two access modifiers:

| Keyword   | Description                            |
|-----------|----------------------------------------|
| readonly  | Marks the storage buffer as read-only  |
| writeonly | Marks the storage buffer as write-only |

If neither modifier is specified, the shader may both read from and write to the storage buffer.

Definition:
```
ssbo_decl ::= "uniform" descriptor_location packing_type? access_modifier? "buffer" identifier: type_specifier ";"
access_modifier ::= "readonly" | "writeonly"
```

#### 4.8.3 Descriptor Locations
A uniform declaration shall specify a descriptor set and binding using the `set` and `binding` specifiers.

The value following `set` specifies the descriptor set number.
The value following `binding` specifies the binding number.

Both values shall be non-negative integer constants.

The pair `(set, binding)` uniquely identifies a descriptor within a shader interface. Two resource declarations shall not specify the same descriptor location.

TODO:
5. Expressions
   5.1 Operators
   5.2 Conversions
   5.3 Function Calls
   5.4 Constructors

6. Statements
   6.1 Blocks
   6.2 Conditional Statements
   6.3 Loops
   6.4 Return Statements

7. Declarations
   7.1 Variables
   7.2 Constants
   7.3 Functions
   7.4 Structures

8. Memory Layout
   8.1 Alignment
   8.2 Size
   8.3 Member Offsets
   8.4 Arrays
   8.5 Matrices

9. Resources
   9.1 Uniforms
   9.2 Storage Buffers
   9.3 Textures
   9.4 Samplers

10. Shader Stages
    10.1 Vertex
    10.2 Fragment
    10.3 Compute
    10.4 Stage Interfaces

11. Compilation
    11.1 Compilation Model
    11.2 Semantic Analysis
    11.3 SPIR-V Generation

12. SPIR-V Mapping

13. Errors and Undefined Behavior

14. Standard Library

Appendix A — Grammar
Appendix B — Built-in Types
Appendix C — SPIR-V Mapping Tables
