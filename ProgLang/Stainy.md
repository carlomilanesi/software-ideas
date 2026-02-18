# Introduction

This is the design document of a new programming language, named **Stainy**.
The other documents in this folder explain its design choices.

# Name of the language

The name "Stainy" has been chosen simply because it looks like an English word, but it is not.
It can be understood as "related to stain", like "brainy" from "brain", or "grainy" from "grain".
So, it is a homage to the Rust programming language, as "stain" and "rust" are semantically related.

# Mutable and immutable objects

At compile time, every object is clearly defined as mutable or immutable.
The language requires that every variable is declared as mutable or as immutable.
Every variable must be initialized to a value, which defines its type.

Here is the declaration and initialization of an immutable floating-point variable named `distance`, having `23.4` as value: `23.4e0 :distance`.

Here is the declaration and initialization of a mutable string variable named `town`, having `London` as value: `"London" :>town`.

Here is the declaration and initialization of an immutable struct variable named `person`, having as fields a floating-point `distance` and a string `town`:
```
    23.4e0 :distance
    "London" :town
:person
```

So, the syntax to define an immutable variable or structure field is: `<expression> :<variable name>`.

And the syntax to define a mutable variable or structure field is: `<expression> ::<variable name>`.

Structs are defined by their indentation.

# Copy semantics

Assignments have share semantics of immutable objects and copy-on-write semantics of mutable objects.

These cases of assignment are possible at compile-time:
* An immutable object is assigned to an immutable object (I => I): Immutable share semantics is applied.
* An immutable object is assigned to a mutable object (I => M): This case is forbidden by the language, and so the compiler should generate a compilation error. This is because an immutable object shouldn't implicitly become mutable. If such a case is really needed, an explicit clone function should be called on the source object, to create a mutable copy to assign to the destination.
* A mutable object is assigned to an immutable object (M => I): For the lifetime of the destination, also the source becomes immutable, and immutable share semantics is implemented; after the end of the lifetime of the destination, the source becomes mutable again. In the typical case the assignment is performed as part of a function call, this is the usual behavior: the variables of the caller cannot be modified, as long the called function is running.
* A mutable object is assigned to a mutable object (M => M): Copy-on-write semantics is applied.

# Numbers

There are ten types of numbers:
* **Float**: IEEE 64-bit floating-point numbers.
* **Decimal[F]**: These are 10 distinct types, with F going from 0 to 9. Each of these types represents numbers with F fractional decimal digits.

There is no overlap between Float literals and Decimal literals. So, a numeric literal can be unambiguously detected as a Float or as a Decimal.

## Floats

Floats can be combined in operations only with other Floats, and they can be assigned only to Float variables. To combine a Decimal with a Float, it must be explicitly previously converted to a Float.

Float literals contain the letter "e" followed by an optional "-" and then an integer number. For example, the number 14.3 can be written as `0.143e2`, or as `1.43e1`, or as `14.3e0`, or as `143e-1`, or as `1430e-2`. The simplest way to write 0, is `0e0`.
Their regex is `[\-][0-9]+[\.][0-9]*e[\-][0-9]+`.

## Decimals

Decimals are fixed point numbers, implemented by a signed or unsigned integer number.

Decimal literals begin with a digit or a minus sign, possibly contain a dot, and contain no letters. Some examples: `0`, `42`, `4.2`, `0004.20`, `-0000042`.

Decimals can be combined in operations and assignments only with other decimals, possibly having a different number of integer digits, but only with the same number of fractional digits.
