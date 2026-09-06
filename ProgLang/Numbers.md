# Number Types

## Systems Programming and Application Programming

A difference between systems languages and application languages is this:
* In systems languages, the available number types are the ones for which arithmetic operations are the ones implemented in hardware, like 32-bit signed integer numbers, or 64-bit floating-point numbers.
* In application languages, the available number types are the ones for which arithmetic operations are implemented by hand in the application domain, like decimal numbers with 8 integer digits and 2 fractional digits.

For example, in application like business management, accounting, (non-electronic) engineering, (non-computer) science, it makes little sense to specify how many bits are used by a number, and hexadecimal numbers are not used.
Instead, in such domains, numbers are specified by their number of decimal digits for the integer part and their number of decimal digits for the fractional part.

The numeric types defined by the languages like C, Java, and Rust are appropriate for systems programming, but not for application programming.
The numeric types defined by the languages like COBOL are appropriate for application programming, but not for systems programming.

## Number Types and Literals for Application Programming

The types needed by application programming are these:
* UnsignedNumber[I.F]: Positive or zero number with at most I integer decimal digits and F fractional decimal digits, with I and F numbers from 0 to 9. The internal representation can be compressed binary-coded decimal or binary fixed point. The second one is much more efficient.
* SignedNumber[I.F]: Similar to a UnsignedNumber, but supporting also negative numbers.
* FloatingPointNumber: IEEE 64-bit floating-point number. This is needed for technical/scientific computations.

Numeric literals can represent such types with no need of casting: 00034.980 represents an unsigned number with 5 integer digits and 3 fractional digits; +00034.980 represents the corresponding signed number; 2000e0 represents two-thousands as a floating-point number.

For most kinds of application programming, 32-bit floating-point numbers and arbitrary precision numbers are not needed.

In case they are actually needed, an external library should be used. The input and output of such a library would be done using the existing number types, or using strings of digits.
