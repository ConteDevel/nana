[Home](README.md)

# Data types

The Nana is a weak-typed language that means it's not necessary to always specify data types but these annotations help to restrict a set of available values in variables and improve stability and self-documentation of the code.

## Bit

The `Bit` is a logical type that accepts values: `TRUE` and `FALSE`.

## Int

The `Int` is an integer type that accepts any integer numbers. Nana has a built-in support of long arithmetics, so `Int` has not explicit limits.

## Num

The `Num` is a type to store real numbers, i.e. `-1.0`, `3.14` or `1.67E-27`. As with `Int` type, Nana supports long arithmetics for `Num`.

## Str

The `Str` allows to store text values, i.e. `'Hello, World!'`, use single quotes (`'`) to write a string value.

## List

A list is an ordered sequence of elements, use `[]` to declare a list data type or create a new list, i.e. `a: [Str] = ['Hello', 'World']`. List indexing starts with zero (`0`), to access a list element use `[]`.

## Tuple

A tuple is a group of logically related elements, use `()` to declare a tuple data type or create a new tuple and `[]` to access a tuple element. As in list case, an indexing starts with zero (`0`). For example:

```
t: (Str, Num) = ('Hello, World!', 3.14)
t[0]
t[1 + 2]
```

## Dictionary

A dictionary is a data structure to store key-value pairs, notice all key must be unique in context of a single dictionary. Use `{}` to declare a dictionary data type or create a new dictionary and `[]` to access a dictionary value by key. For example:

```
d: {Str: Int} = {
    'Key1': 1,
    'Key2': 2
}
d['Key1']
```

## Constrained dictionary

Nana allows to specify keys explicitly as it's shown below:  

```
d1: {key1: Int, key2: Int} = {
    key1: 1,
    key2: 2
}
d2: {'key1': Int, 'key2': Int} = {
    'key1': 1,
    'key2': 2
}
d3: {1: Bit, 0: Bit} = {
    1: TRUE,
    0: FALSE
}
```

Basing on previos samples the next expression is valid:

```
d4: {'Key1': Int, 'Key2': Int} = d
```

This is invalid:

```
d4: {Key1: Int, Key2: Int} = d
```

because of there are named keys in `d4` instead of `Str` keys in `d`.

## Domain

A domain is a special Nana data type that allows to create complex types or can be used as an alias, use `,` to enumerate available types and `<>` to declare a new domain, `<>` is not required when the domain is used as an alias. For example:

```
Gender = <male, female>
User = {
    name: Str,
    email: Str,
    gender: Gender,
    age: Int
}
Bot = {
    id: Int,
    name: Str
}
Client = <User, Bot>
```

## Fun

A `Fun` data type allows to declare a function signature, it can be used to restrict a set of functions that can be passed to variable or as an argument of called function. Use a `(type) -> type` schema to declare `Fun` where the first type is arguments and the last is a returned value of the function. By default Nana function returns a tuple with zero `(0)`. Look at examples below to learn more:

```
foo() {
    ret ()
}

boo(a: Str) -> Bit {
    ret FALSE
}

callbackFoo: () -> () = foo
callbackBoo: (Str) -> Bit = boo
```

## Any

`Any` is a dynamic type that can be linked with any other type, use it when variable or function argument must be able to accept any type. Notice that's not required to explicitly specify `Any` type, by default if other type is not proved Nana marks a variable as `Any` type. For example:

```
a: Any = 'Hello, World!'
a = 3.14
# Or
b = 'Hello, World!'
b = 3.14
```

[Home](README.md)
