# **VKSL Language Specification**
### **Version 0.1**

## Abstract
VKSL (VKE Shader Language) is a statically typed GLSL-style language designed for writing GPU programs targeting 
the SPIR-V intermediate representation. The language aims to simplify shader development by removing unnecessary 
explicitness while retaining control over program behaviour and resource representation. 
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
| if             | Conditional execution                                                       |
| else           | Defines the alternative branch of a conditional statement                   |
| while          | Defines a while loop                                                        |
| for            | Defines a for loop                                                          |
| return         | Returns from a function                                                     |
| let            | Declares a variable                                                         |
| include        | Includes a different module                                                 |
| extension      | Defines the usage of an extension                                           |
| input          | Declares a vertex input variable                                            |
| output         | Declares a fragment output variable                                         |
| provide        | Declares an input to the fragment shader, that is not defined using `input` |
| push_constants | Declares a push constants block                                             |
| uniform        | Declares a uniform                                                          |

### 3.5 Built-in Variables
VKSL defines a set of built-in variables that provide access to functionality or resources supplied by the shader
environment.

Built-in identifiers beginning with `VKSL_` are reserved and shall not be redeclared by a VKSL program.

The following built-in identifiers are defined:

| Identifier    | Purpose                                                   |
|---------------|-----------------------------------------------------------|
| VKSL_position | Specifies the output position of the vertex shader stage  |

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

TODO:
4. Types
   4.1 Scalar Types
   4.2 Vector Types
   4.3 Matrix Types
   4.4 Arrays
   4.5 Structures
   4.6 Resource Types

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
