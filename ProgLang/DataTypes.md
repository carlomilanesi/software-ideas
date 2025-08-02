# DataTypes

## Advantages of variable declarations

Some languages, like Python and JavaScript, allow to use variables never declared before.
This appears to be an advantage, at least for quick-and-dirty coding, but it has actually several important disadvantages, so that even some languages having dynamic typing, like SmallTalk, force to declare variables before using them.

These are the advantages of using a programming language which forces programmers to declare variables before using them, in order from the most important to the least important:
1. **Detecting typos in variable names**. If, in a language which does not require variable declarations, a variable named, say, `content` is used in a statement, and a variable named `contents` is used in another statement, the semantics of this code is that two distinct variables will be used, after having implicitly declared them. Though, probably the developer meant to use just one variable. Having two distinct names is just a typo, that will not be detected by the language implementation nor by static checkers. Instead, using a language requiring the declaration of variables, if only the variable named `content` has been declared, the use of the variable `contents` will cause a syntax error.
2. **Being able to create inner scopes**. If, in a language which does not require variable declarations, a variable is used in a scope, the only way to use another variable in an inner scope is by using a different name. Instead, in a language which does require variable declaration, another variable with the same name can be declared in an inner scope. The inner-scope variable will shadow the outer-scope variable.
3. **Having a place where variables can be documented**. If variables declarations are not required, it is not clear where the comment describing a variable should be put. Instead, if a declaration is required, it is obvious that such a comment should be put in the line before the one in which it is declared, or in the same line.

## Advantages of static datatypes

In the so-called *dynamic-typed languages*, like Python, JavaScript, SmallTalk, any variable can be assigned first a value of numeric type, and then a value of string type. The type of any variable can change at any assignment to it.

Instead, in the so-called *static-typed languages*, like C, C++, Java, and Rust, if any variable is assigned first a value of numeric type, and then a value of string type, the compiler with certainly emit a compilation error. Any variable has a fixed datatype, determined at compilation time.

Actually, this is not always true even for static-typed languages, because of the infamous null pointer, available in C, C++, Java, and unsafe Rust. If a pointer variable has value `null`, some operations on that pointer have no more meaning, an so its effective type is different than when the pointer is not null. But let's set aside this case.

Here are the advantages of static typing over dynamic typing:
1. **Type-safety**. A type determines which operations are allowed on a variable. If the compiler must be able to determine the type of a variable, it can be able also of determining which operations are avalailable on every variable. If the code contains an avalailable operation, it is a programming error, and the compiler is able to detect such an erro.
2. **Memory performance**. A type determines how much memory can be used by a variable. If the compiler must be able to determine the type of a variable, it can be able also of determining how much memory to allocate for every variable. So, the compiler can allocate exactly the needed memory, often on the stack, which is way faster than the heap. If the compiler cannot detect the type of a variable, it can allocate only a small generic structure, containing a pointer to the actual data. The actual data is necessarily allocated in the heap when the variable is created.
3. **Speed performance**. A type determines the bit sequence used by a variable. If the compiler must be able to determine the type of a variable, it can be able also of determining which machine language operations to use to manipulate the bit pattern for every variable. So, the compiler can generate exactly the needed machine code for every variable. If the compiler cannot detect the type of a variable, it must check the type of the variable at runtime, and jump to the machine code routine specific for that datatype.

## Advantages of algebraic datatypes

## Advantages of subenums
