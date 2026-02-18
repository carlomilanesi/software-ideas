# Introduction

This is the design document of a new programming language, named **Stainy**.
The other documents in this folder explain its design choices.

# Name

The name "Stainy" has been chosen simply because it looks like an English word, but it is not.
It can be understood as "related to stain", like "brainy" from "brain", or "grainy" from "grain".
So, it is a homage to the Rust programming language, as "stain" and "rust" are semantically related.

# Mutable and immutable objects

At compile time, every object is clearly defined as mutable or immutable.
The language requires that every variable is declared as mutable or as immutable.

# Copy semantics

Assignments have share semantics of immutable objects and copy-on-write semantics of mutable objects.

These cases of assignment are possible at compile-time:
* An immutable object is assigned to an immutable object (I => I): Immutable share semantics is applied.
* An immutable object is assigned to a mutable object (I => M): This case is forbidden by the language, and so the compiler should generate a compilation error. This is because an immutable object shouldn't implicitly become mutable. If such a case is really needed, an explicit clone function should be called on the source object, to create a mutable copy to assign to the destination.
* A mutable object is assigned to an immutable object (M => I): For the lifetime of the destination, also the source becomes immutable, and immutable share semantics is implemented; after the end of the lifetime of the destination, the source becomes mutable again. In the typical case the assignment is performed as part of a function call, this is the usual behavior: the variables of the caller cannot be modified, as long the called function is running.
* A mutable object is assigned to a mutable object (M => M): Copy-on-write semantics is applied.

# Numbers

There are two types of numbers:
* **Float**: IEEE 64-bit floating-point numbers. Float literals contain the letter "e" followed by an optional "-" and then an integer number. For example, the number 14.3 can be written as `0.143e2`, or as `1.43e1`, or as `14.3e0`, or as `143e-1`, or as `1430e-2`. The simplest way to write 0, is `0e0`.
* **Decimal[I.F]**: Numbers with at most I integer decimal digits and F fractional decimal digits, with I and F numbers from 0 to 9.

Floats can be combined in operations and assignments only with other floats.
Decimals can be combined in operations and assignments only with other decimals, possibly having a different number of integer digits, but only with the same number of fractional digits.
