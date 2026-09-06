# DataTypes

## Advantages of variable declarations

Some languages, like Python and JavaScript, allow to use variables never declared before.
This appears to be an advantage, at least for quick-and-dirty coding, but it has actually several important disadvantages, so that even some languages having dynamic typing, like SmallTalk, force to declare variables before using them.

These are the advantages of using a programming language which forces programmers to declare variables before using them, in order from the most important to the least important:
1. **Detecting typos in variable names**. If, in a language which does not require variable declarations, a variable named, say, `content` is used in a statement, and a variable named `contents` is used in another statement, the semantics of this code is that two distinct variables will be used, after having implicitly declared them. Though, probably the developer meant to use just one variable. Having two distinct names is just a typo, that will not be detected by the language implementation nor by static checkers. Instead, using a language requiring the declaration of variables, if only the variable named `content` has been declared, the use of the variable `contents` will cause a syntax error.
2. **Being able to create inner scopes**. If, in a language which does not require variable declarations, a variable is used in a scope, the only way to use another variable in an inner scope is by using a different name. Instead, in a language which does require variable declarations, another variable with the same name can be declared in an inner scope. The inner-scope variable will shadow the outer-scope variable.
3. **Having a place where variables can be documented**. If variables declarations are not required, it is not clear where the comment describing a variable should be put. Instead, if a declaration is required, it is obvious that such a comment should be put immediately before or immediately after that declaration.

## Advantages of static datatypes

In the so-called *dynamic-typed languages*, like Python, JavaScript, or SmallTalk, any variable has no fixed type defined at compile time; such a variable can be assigned first a value of numeric type, and then a value of string type. The type of any variable can change at any assignment to it.

Instead, in the so-called *static-typed languages*, like C, C++, Java, and Rust, every variable has a fixed type, determined at compile time; if such a variable is assigned first a value of numeric type, and then a value of string type, the compiler with certainly emit a compilation error.

Actually, this is not always true even for static-typed languages, because of the infamous null pointer, available in C, C++, Java, and unsafe Rust. If a pointer variable has value `null`, some operations on that pointer have no more meaning, an so its effective type is different than when the pointer is not null. But let's set aside such a case.

Here are the advantages of static typing over dynamic typing:
1. **Type-safety**. A type determines which operations are allowed on a variable. If the compiler must be capable of determining the type of any variable, it can also determine which operations are available on every variable. If the code a call to an unavailable operation, such call is a programming error, and the compiler can detect such an error.
2. **Memory performance**. A type determines how much memory can be used by a variable. If the compiler must be capable of determining the type of any variable, it can also determine how much memory to allocate for every variable. So, the compiler can allocate exactly the needed memory, often on the stack, which is way faster than the heap. If the compiler cannot detect the type of a variable, it can allocate only a small generic structure, capable of containing the data of some simple types, or a pointer to the actual data, allocated in the heap at runtime, when the variable is created.
3. **Speed performance**. A type determines the bit sequence used by a variable. If the compiler must be capable of determining the type of a variable, it can also determine which machine language operations to use to manipulate the bit sequence belonging to every variable. So, the compiler can generate exactly the needed machine code for every variable. If the compiler cannot detect the type of a variable, it must generate instructions which check the type of the variable at runtime, and jump to the machine code routine specific for that datatype.

## Advantages of algebraic datatypes

In very most programming languages, there is a data type feature, usually named "class" or "struct", whose instances are sequences of named fields, each having a specific datatype. For example:
```
class Point {
    x: Number;
    y: Number;
}
```

In most programming languages, there is also a data type feature, usually named "enum", whose instances are values belonging to a set of allowed named integer values. For example:
```
enum Direction {
    North, // = 0
    South, // = 1
    West, // = 2
    East, // = 3
}
```

In not-so-many programming languages, enums are not restricted to be integer values. In addition to a named integer tag, they can have any additional members. Such types are named *algebraic data types*. For example:
```
enum Event {
    Mouse {
        button: Button;
        position: Point;
    },
    Key {
        keycode: Integer;
    },
    Activate,
    Deactivate,
}
```

Algebraic data types allow to specify in a safe and efficient way cases in which a value can represent different unrelated types. The alternative implementation, used by object-oriented languages, is to have a base class and several derived classes; though, such object-oriented architecture can be more verbose and less efficient.

This can be expressed by the following object oriented code:

```
class Event {}

class Key: Event {
    keycode: Integer,
}

class Mouse: Event {
    button: Button;
    position: Point;
}

if let key_event = event.downcast<Key>() then ...
else if let mouse_event = event.downcast<Mouse>() then ...
```

Equivalently, using the enum, this can be expressed by this code:

```
match event {
    Key { keycode } => ...
    Mouse { button, position } => ...
}
```
