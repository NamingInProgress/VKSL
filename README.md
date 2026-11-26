# VKSL compiler

The compiler for the custom VKSL shading language for VKEngine.
There should be docs but our fellow Vulkan Volunteer Mr. Bear does
not appear to be in the mood to do them.

## This project is 0% AI

This is a hobby project and I love compilers. I have not and will not use AI to
both write code or even get ideas for stuff. Everything is done all by myself.


# Unstable features
(mocking rust)

Making a vec4 out of two vec2s can be confusing and is not the most intuitive, so it is locked behind an unstable feature
```
#use_unsable_feature(vec4_from_single_vec2)
const a: vec2 = vec2(1.0);
const b: vec4 = vec4(a);
```

Matrix multiplication and linear algebra is super unintuitive, so as to not confuse users, its locked behind an unstable feature
```
#use_unstable_feature(matrix_multiplication)
const a: mat4 = mat4(vec4(1.0));
const b: vec4 = vec4(1.0);
const c: vec4 = a * b;
```

Addition through the add operator should be stable, but our PR for stabilization has gone stale :( and nobody has checked it for a while, there are the following unresolved questions: []
```
#use_unstable_feature(addition)
const a: int = 10;
const b: int = 20;
const c: int = a + b;
```
For now, use the add() function as a simple workaround:
```
const a: int = 10;
const b: int = 20;
const c: int = add(a, b);
```