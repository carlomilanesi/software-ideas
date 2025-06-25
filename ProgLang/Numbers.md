# Number Types

## Systems Programming and Application Programming

A difference between systems languages and application languages is that in systems languages the available number types are the ones for which arithmetic operations are the ones implemented in hardware, and in application languages the available number types are the ones for which arithmetic operations are implemented by hand in the application domain.

For example, in application domains like business management, accounting, (non-electronic) engineering, (non-computer) science, there are integer numbers, but usually there not such things like 32-bit numbers, or numbers in hexadecimal notation. Instead, in such domains there are numbers with a given number of decimal digits for the integer part and a given number of decimal digits for the fractional part.

The numeric type defined by the Rust language are appropriate for systems programming language, but not for application programming.

## Number Types and Literals for Application Programming

The types needed by application programming are these:
* Number[I.F]: Number with at most I integer decimal digits and F fractional decimal digits, with I and F numbers from 0 to 9. The internal representation can be compressed binary-coded decimal or binary fixed point.
* Float: IEEE 64-bit floating-point number.

Numeric literals can represent such type with no need of casting: 00034.980 represents a number with 5 integer digits and 3 fractional digits; 2000e0 represents two-thousands as a floating-point number.

For most kinds of application programming, 32-bit floating-point numbers and arbitrary precision numbers are not needed.

In case they are actually needed, an external library should be used. The input and output of such library would be done using the existing number types, or using strings of digits.
