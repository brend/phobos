# Phobos

Phobos is a programming language designed for simplicity and clarity. It is a statically typed language that supports a variety of data types and control structures.

Included is a transpiler that can convert Phobos code into Lua:

``` phobos
fn inc(n: Number): Number { return n + 1; }
```

This transpiles to Lua:

``` Lua
function inc(n)
    return (n + 1)
end
```
