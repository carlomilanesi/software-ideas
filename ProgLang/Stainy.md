# Introduction

This is the design document of a new programming language, named **Stainy**.
The other documents in this folder explain its design choices.

# Name of the language

The name "Stainy" has been chosen simply because it is easy to pronounce in English, but quite rare in usage.
It can be understood as "related to stain".
So, it is a homage to the Rust programming language, as "stain" and "rust" are semantically related.

# Capitalization

Identifiers of types and type classes ("interfaces" or "traits" in other languages) must start with an uppercase letter.
Every other identifier must start with a lowercase letter.
The rest of the identifier must use *camel case*, like in Java language.

Two identifiers in the same scope cannot differ only by capitalization of a non-initial letter.
For example, it is allowed to have in the same scope the identifiers `Total`, `Totalvalue`, `total`, and `totalValue`, because the first two are types and the second two are not types.
Though, it is not allowed to have in the same scope the identifiers `totalvalue` and `totalValue`, because they are both non-types with very similar names.

# Comments and attributes

The portion of the line starting with `#` is considered a comment. There are no other types of comments.

If a line starts with a single quote, it is an attribute applied to the following item.
Here are some examples of attributes:
```
'if os = "Linux"
''
```
The first one is a conditional compilation. It specify to process the following item only if the build directive "os" has value "Linux".
The second one is a documentation comment for the following item.

If a line starts with a caret, it is an attribute applied to the containing item.
Here are some examples of attributes:
```
^if os = "Linux"
^'
```
The first one is a conditional compilation. It specify to process the enclosing item only if the build directive "os" has value "Linux".
The second one is a documentation comment for the enclosing item.

# Mutable and immutable objects

At compile time, every object is clearly defined as mutable or immutable.
The language requires that every variable is declared as mutable or as immutable.
Every variable must be initialized to a value, which defines its type.

Here is the declaration and initialization of an immutable 64-bit floating-point variable, named `distance`, having `23.4` as its value: `23.4e0 :distance`.

Here is the declaration and initialization of a mutable string variable named `town`, having `London` as its initial value: `"London" ::town`.

Structs must have explicit types.
Here is the declaration of a struct type named `Person`, having as fields the floating-point member `distance` and the string member `town`:
struct Person
    distance Float
    town String

Here is the declaration and initialization of an immutable struct variable named `person`, having as fields the floating-point member `distance` and the string member `town`:
```
Person
    23.4e0 :distance
    "London" :town
:person
```

Here is the declaration and initialization of an immutable struct variable named `person`, having as fields the floating-point member `distance` and the string member `town`:
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

There is no overlap between Float literals and Decimal literals. Every Float literal contains the letter "e", while no Decimal literal contains that letter. So, any numeric literal can be unambiguously detected as a Float or as a Decimal.

## Floats

Floats can be combined in operations only with other Floats, and they can be assigned only to Float variables. To combine a Decimal with a Float, it must be explicitly previously converted to a Float.

Decimal literals begin possibly by a minus sign, followed by one or more digits, followed possibly by a dot, followed possibly by some digits (for the fractional part), followed by the letter "e", followed possibly by a minus sign, followed by one or more digits. For example, the number 14.3 can be written as `0.143e2`, or as `1.43e1`, or as `14.3e0`, or as `143e-1`, or as `1430e-2`. The simplest way to write 0, is `0e0`.
Their regex is `[\-][0-9]+[\.][0-9]*e[\-][0-9]+`.

## Decimals

Decimals are fixed point numbers, implemented by a signed integer number. For example, the number `23.456`, having type `Decimal[3]`, is implemented by the integer number 23456.

Decimal literals begin possibly by a minus sign, followed by one or more digits, followed possibly by a dot, followed possibly by some digits (for the fractional part). These literals contain no letters. Some examples: `0`, `42`, `4.2`, `0004.20`, `-0000042`. Their regex is `[\-][0-9]+[\.][0-9]*`.

In Decimal literals, the number of digits after the dot determine the type. For example, `3` is of type `Decimal[0]`, but `3.00` is of type `Decimal[2]`.

Decimals can be combined in operations and assignments only with other Decimals. In addition, Decimals can be assigned only to Decimals of the same number of fractional digits, and they can be compared, added, or subtracted only with Decimals having the same number of fractional digits. Instead, the multiplication operation of two Decimals generates a Decimal whose number of fractional digits is the sum of the number fractional digits of the two operands; and the division and the remainder operations of two Decimals generates a Decimal whose number of fractional digits is the subtraction between the number fractional digits of the two operands.

For example, while result of the division between `1.234e0` and `2.3e0` is `0.536521739`, the result of the division between `1.234` and `2.3` is `0.53`, because the first operand has three fractional digits, the second operand has one fractional digit, and so the result has 3-1=2 fractional digits. Such result is truncated towards zero.

# Arrays

The language defines only one list type, that allows a dynamic number of items, with the type of such items which must be defined at compile-time, similarly to C++'s `std::vector` or Rust's `Vec`.

This is the definition and initialization of an immutable array variable, containing three strings, and of a mutable array, initially containing one Float:
```
["one", "two", "three",] :strings
[42e0,] ::numbers
```

Notice that every item must be followed by a comma.
The types of such arrays is inferred.

Here are some statements which manipulate such arrays.
```
(strings 1) get # returns the second element of `strings`, panics if there is not such an element
(numbers 0 1.2e0) set # assigns the value `1.2` to the first element of `numbers`, panics if there is not such an element
(numbers 2.3e0) push # adds an element, having value `2.3`, at the end of the array `numbers`
(numbers) pop # removes and returns the last element of `numbers`, panics if there is not such an element

```


# Dictionaries

The language defines two dictionary types, one ordered and one unordered. They allow a dynamic number of key-value associations, but the type of keys and values must be defined at compile-time, similarly to C++'s `std::map` and `std::unordered_map` or Rust's `BTreeMap` and `HashMap`.

This is the definition and initialization of an ordered dictionary variable, containing three Decimal-to-string associations, and of an unordered dictionary variable, containing one string-to-Float associations:
```
{< 1.00 "one", 2.00 "two", 3.00 "three",} :decimal_to_string
{# "three" 3e0,} :strings_to_float
```

Notice that every item must be followed by a comma.

# Loops

There are two kinds of loops:
* **do-loops**, which iterate over an iterator.
* **infinite loops**, which iterate forever, until a statement breaks out of the loop.
Here is an example of a do-loop:
0:9 => 
    
