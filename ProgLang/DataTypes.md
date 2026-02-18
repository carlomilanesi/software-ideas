# DataTypes

## Advantages of variable declarations

Some languages, like Python and JavaScript, allow to use variables never declared before.
This appears to be an advantage, at least for quick-and-dirty coding, but it has actually several important disadvantages, so that even some languages having dynamic typing, like SmallTalk, force to declare variables before using them.

These are the advantages of using a programming language which forces programmers to declare variables before using them, in order from the most important to the least important:
1. **Detecting typos in variable names**. If, in a language which does not require variable declarations, a variable named, say, `content` is used in a statement, and a variable named `contents` is used in another statement, the semantics of this code is that two distinct variables will be used, after having implicitly declared them. Though, probably the developer meant to use just one variable. Having two distinct names is just a typo, that will not be detected by the language implementation nor by static checkers. Instead, using a language requiring the declaration of variables, if only the variable named `content` has been declared, the use of the variable `contents` will cause a syntax error.
2. **Being able to create inner scopes**. If, in a language which does not require variable declarations, a variable is used in a scope, the only way to use another variable in an inner scope is by using a different name. Instead, in a language which does require variable declarations, another variable with the same name can be declared in an inner scope. The inner-scope variable will shadow the outer-scope variable.
3. **Having a place where variables can be documented**. If variables declarations are not required, it is not clear where the comment describing a variable should be put. Instead, if a declaration is required, it is obvious that such a comment should be put immediately before or immediately after that declaration.

## Advantages of static datatypes

In the so-called *dynamic-typed languages*, like Python, JavaScript, or SmallTalk, any variable can be assigned first a value of numeric type, and then a value of string type. The type of any variable can change at any assignment to it.

Instead, in the so-called *static-typed languages*, like C, C++, Java, and Rust, if any variable is assigned first a value of numeric type, and then a value of string type, the compiler with certainly emit a compilation error. Any variable has a fixed datatype, determined at compilation time.

Actually, this is not always true even for static-typed languages, because of the infamous null pointer, available in C, C++, Java, and unsafe Rust. If a pointer variable has value `null`, some operations on that pointer have no more meaning, an so its effective type is different than when the pointer is not null. But let's set aside this case.

Here are the advantages of static typing over dynamic typing:
1. **Type-safety**. A type determines which operations are allowed on a variable. If the compiler must be capable of determining the type of a variable, it can determine which operations are avalailable on every variable. If the code contains an unavalailable operation, it is a programming error, and the compiler can detect such an error.
2. **Memory performance**. A type determines how much memory can be used by a variable. If the compiler must be capable of determining the type of a variable, it can also determine how much memory to allocate for every variable. So, the compiler can allocate exactly the needed memory, often on the stack, which is way faster than the heap. If the compiler cannot detect the type of a variable, it can allocate only a small generic structure, containing a pointer to the actual data. The actual data is necessarily allocated in the heap at runtime, when the variable is created.
3. **Speed performance**. A type determines the bit sequence used by a variable. If the compiler must be capable of determining the type of a variable, it can also determine which machine language operations to use to manipulate the bit sequence belonging to every variable. So, the compiler can generate exactly the needed machine code for every variable. If the compiler cannot detect the type of a variable, it must generate instructions which check the type of the variable at runtime, and jump to the machine code routine specific for that datatype.

## Advantages of algebraic datatypes

In very most programming languages, there is a data type feature, usually named "class" or "struct", in which an instance is a sequence of named fields, each having a specific datatype.

In most programming languages, there is also a data type feature, usually named "enum", in which an instance is a value belonging to a set of allowed named integer values.

In not-so-many programming languages, enums are not restricted to be integer values, but in addition to a named integer tag, they can have any additional members. Such types are named *algebraic data types*.

Algebraic data types allow to specify in a safe and efficient way cases in which a value can represent different unrelated types. The alternative implementation, used by object-oriented languages, is to have a base class and several derived classes; though, this is more verbose and not- so efficient.

For example, consider the case in which there can be two events: a key pressed, specifying also which key has been pressed, and a mouse click, specifying also which mouse button has been clicked, and in which position.

This can be expressed by the following object oriented code:

```
class Event {}

class KeyPress: Event {
    key: KeyCode,
}

class MouseClick: Event {
    button: MouseButton,
    x: int,
    y: int,
}

if let key_event = event.downcast<KeyPress>() then ...
else if let mouse_event = event.downcast<MouseClick>() then ...
```

Equivalently, this can be expressed by this code:

```
enum Event {
    KeyPress {
        key: KeyCode
    }
    MouseClick {
        button: MouseButton,
        x: int,
        y: int,
    }
}

match event {
    KeyPress { key } => ...
    MouseClick { button, x, y } => ...
}
```

## Advantages of subtypes

In programming languages, a quite rare feature is that of subtype.

Let's assume that in an application there is the need to have the types `UIEvent` and `KeyboardEvent`.
We have that every value of `KeyboardEvent` is also a valid value of `UIEvent`, and every operation supported by `KeyboardEvent` is also supported by the type `UIEvent`, with the same semantics.

One way to implement this is to declare two distinct types, with their operations.
This has these disadvantages:
* Every operation must be defined twice.
* The infallible conversion of a KeyboardEvent to an UIEvent and the fallible conversion of a UIEvent to a KeyboardEvent must be defined explicitly.

Another way is to use the object-oriented programming paradigm.
A class defines the type UIEvent, and a class derived from it defines the type KeyboardEvent.
This can be applied to structures, but not to enums.

Another way is to add a field which specifies to which set the current istance belongs to.
For example, the class UIEvent, can have the Boolean field `is_keyboard_event`.
An alternative is to have the enum field `input_device`, having `Keyboard` as one of its variants.
Also this one can be applied to structures, but not to enums.

If there is an enum which describes all the possible UIEvents, it is useful to have another enum which describes all the possible KeyboardEvents.
A way to implement this is to mark every variant of the enum UIEvents by a list of attributes which define to which other enums that variant belongs.
Such other enums are subenums, that is a useful kind of subtyping.
