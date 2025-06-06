# Phobos

Phobos is a programming language designed for simplicity and clarity. It is a statically typed language that supports a variety of data types and control structures.

Included is a transpiler that can convert Phobos code into Lua. For example, if you have a Phobos script named `script.pho`:

``` phobos
fn inc(n: Number): Number { return n + 1; }
```

The command `cargo run script.pho` will write the following Lua code to the console:

``` Lua
function inc(n)
    return (n + 1)
end
```

## Roadmap

The following features are planned for the first release:

## Record types

### Record type definitions

```
record Person {
  name: String,
  age: Number
}
```

### Record literals

```
let johnny: Person = { name: "Johnny", age: 42 }
```

### Field access

```
print(johnny.name)
```

## Arrays

### Array literals

```
let numbers: [Number] = [1, 2, 3]
```

### Array indexing

```
print(numbers[1])
```
