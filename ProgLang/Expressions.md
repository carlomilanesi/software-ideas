# Syntax of expressions

## Postfix notation

The vast majority of programming languages use infix binary operators, like in the expression `a + b`, and prefix function call, like in `f(x)` or `(f x)` or simply `f x`.

The only common exception is for method calls, like in `a.f(b)`, in which the function name is put after the first argument, and before all the other arguments.

The disadvantage of the infix and of the prefix notations appears in expression like this ones: `f(g())`. In such a case, first the function `g` must be called, and then function `f`. The eye, moving from left to right, first encounters the name of the function to call last. So, the reading order is not the same as the evaluation order.

Infix operators with precedence have other additional disadvantages:
* Sometimes a difference order of evaluation is needed, and so parentheses must be used to force the desired order.
* In many languages precedence rules are so complex that many developers prefer to add parentheses to make sure the actual order is the desired one, even if they are not needed.

Instead, it is easier to read code in which the reading order is the same as the evaluation order, no precedence rules are needed, and no parentheses to force precedence are needed. This happens only in postfix notation. So, the following expressions are better:
* `a b +`, instead of `a + b` or `(+ a b)`;
* `x y f`, instead of `f(x, y)` or `x.f(y)`.

Such syntax is actually used by the two most famous postfix languages, Forth and PostScript.

## Avoiding explicit stack 

The most famous postfix languages, Forth and PostScript, make explicit use of the process stack. For example, the code `11 22 33 +` means "push on the stack the values 11, 22, 33, and then call the `+` operation on the two top values of the stack, leaving on the stack the initial first value, 11, and the result of the addition, 55.

This two languages are strictly stack-based, and this is rather error-prone and not very readable.
For example, for the expression `11 22 33 +`, it is not clear whether the value `11` is actually needed later or it is an error, and how many operators are used by the `+` operator.

A possible variant to the language is to enclose the arguments in parentheses, in this way: `11 (22 33 +)`, or in this way: `11, (22, 33)+`.
The first notation is LISP-like, but postfix.
The second notation is Pascal-like, but postfix.

Such notations improve readability, because it is clear which are the arguments of the call to `+`; they are more robust, because if the wrong number of arguments is passed, a parsing error is generated; and the result parsing error message can be more clear.

## Functions calls vs function references

In every non-trivial programming language, a function can receive as argument one or more references to functions.
So, whenever the name of a function with no arguments is encountered, it is needed some syntax to distinguish the case in which we want _to call that function_, from the case in which we want _to pass a reference to that function_.

There are three reasonable choices:
1. To use just the name of the function to specify a call, and a decoration to specify a reference.
2. To use just the name of the function to specify a reference, and a decoration to specify a call.
3. To use a decoration of the name of the function to specify a reference, and a different decoration to specify a call.

For example, let's assume we have a function named `f` with no arguments, and a function named `g` receiving as argument a reference to function.
In addition, assume the symbol `@`, prefixed to a function name, means that we want to call that function, and the symbol `&`, prefixed to a function name, means that we want to get a reference to that function.

Using choice 1, the expression `f g` means "first call `f`, and then call `g`, passing as argument the result of the call to `f`".
And the expression `&f g` means "call `g`, passing as argument a reference to `f`".

Using choice 2, the expression `@f @g` means "first call `f`, and then call `g`, passing as argument the result of the call to `f`".
And the expression `f @g` means "call `g`, passing as argument a reference to `f`".

Using choice 3, the expression `@f @g` means "first call `f`, and then call `g`, passing as argument the result of the call to `f`.
And the expression `&f @g` means "call `g`, passing as argument a reference to `f`.

What should be considered is this:
* Conceptually, calling a function is something more complex than taking the reference to that function, and so it deserves a more complex notation, like in choice 2.
* In typical applications, calls to functions are much more common than getting references to functions, and so choice 1 is a more concise notation.
* In many languages, the decoration to specify a function call is just the pair of parentheses enclosing the possible arguments. Such parentheses are anyway useful for readability, to make clear where the list of arguments ends (for prefix notation) or begins (for postfix notation).

## Object-orientation

In object-oriented languages, functions (named "methods") have an automatic scope defined by the type of the first argument.
For example, the expression `a.f(b)` means "call the function `f` defined for the type of `a`, and pass to such a functions two arguments: the value of `a` and the value of `b`".
So, the first argument is prefix, but the possible other arguments are postfix.

Even object-oriented languages have functions that are not methods. For pure object-oriented languages, they are named "class methods" or "static member functions", but for hybrid languages have also free-standing function, i.e. not encapsulated in classes.

In a completely postfix language, a syntax like `(a b)f` is ambiguous, because it could mean:
* Call the method `f` of the object `a`, and pass also the argument `b` to such a call.
* Call the free-standing function `f` with the two arguments `a` and `b`.

To avoid such ambiguity, the method call can have the syntax `a(b)f`.

## Specifying argument names at function calls

In most programming languages, this is an allowed syntax to call a function: `f(12, 7)`. That function could be defined to have the first argument with name `x`, and the second argument with name `y`. At the call statement, such argument names could be displayed by the editor in a tooltip, but they are not shown in source code. Some languages allow to optionally specify the names of the arguments.

Having function calls which display the names of the arguments is a big readability improvement, like when a structure is constructed by specifying the names of the fields, instead of just specifying a list of values in a tuple.

## Function call syntax

Considering what is written above, I think that a good syntax is the following one.

There is a syntax to specify the instance of a structure like this one: `( 12 :x, 7 :y )`. It can be understood as this: "We are going to define a structure. We evaluate the literal `12`. We assign the generated value to a new variable, named `x`. We evaluate the literal `7`. We assign the generated value to a new variable, named `y`. We finish the structure and evaluate it". This structure value can be assigned to a variable of the appropriate type. 

In addition, structure instance specification can be followed by the name of a function, like in this expression: `( 12 :x, 7 :y ) f`. This expression is a call to the function `f`, passing `12` for the argument `x`, and `7` for the argument `y`. Actually, it can be interpreted also as a call to the function `f`, passing just one argument, which is a structure with two fields. So, all functions receive only one argument, which must be a structure.

Consider instead this expression: `( f :func ) g`. This is a call to the function `g`, passing to it one argument, named `func`, and having a reference to the function `f` as value.

Consider instead this other expression: `( ( ) f :func ) g`. To evaluate this expression, first the `f` function is called, with no arguments, and then the function `g` is called, passing to it the result of the call to `f`.

So, every time the name of a function appears in code, that means a reference of that function. But if that name is preceded by a structure, that function is called, passing that structure as its only argument.

So, to perform an operation as simple as the sum of the two literal numbers `5` and `7`, while in most programming languages it is allowed to write simply `5 + 7`, the notation described above would require to write `( 5 :addend1, 7 :addend2 ) add`. A possible alternative implementation is to use significant indentation, like this code:
```
    5 :addend1
    7 :addend2
add
```

For a bigger example, consider the following code in C++: `auto a = 5 + 7 * 12; auto b = 8 - 22 / 3`.

It can be converted into this code:
```
    5 :addend1
        7 :factor1
        12 :factor2
    multiply :addend2
add :a
    8 :minuend
        22 :dividend
        3 :divisor
    divide :subtrahend
subtract :b
```

A different notation should be used for overwriting an existing variable. For a example, consider the following code in C++: `auto a = 5 + 7 * 12; a = 8 - 22 / 3`.

It can be converted into this code:
```
    5: addend1
        7: factor1
        12: factor2
    multiply: addend2
add: a
    8: minuend
        22: dividend
        3: divisor
    divide: subtrahend
subtract ->a
```

So, the symbol `:` means that a new name is introduced and initialized. The name following the colon character can represent a variable or a structure field (or a function argument, which is just the same as a structure field). Instead, the symbol `->` means that an existing variable is reassigned.

## Collections

Structures should be typed, meaning that the type of a structure determines how many fields it has, and it determines also which are the types of such arguments.

Also functions should be typed, meaning that the type of a function determines the type of the structure received as argument, and the type of the structure returned by the function.

So, it appears impossible to create a function with a variable number of arguments. Actually, this is possible by passing a collection in a field of the argument.

Le's assume we want to create a function `f` receiving a variable number of strings as argument.
First, we must create a collection containing such strings.
In C++, we could write this:
```
auto three_items = vector<string>();
three_items.push_back("one");
three_items.push_back("two");
three_items.push_back("three");
```
Instead, with our notation, it could be done with this code:
```
( ( ( ( ) Array[String].new :collection, "one": item ) push :collection, "two": item ) push :collection, "three": item ) push :three_items
```
or, alternatively, this code:
```
        
            Array[String].new :collection
            "one": item
        push :collection
        "two": item
    push :collection
    "three": item
push :three_items
```
This is clearly not clear enough, and so a sintax sugar should be added, to allow this:
```
{ "one", "two", "three" } :three_items
```

This allows to write our function as this:
```
( { "one", "two", "three" } :arg ) f
```

Another useful syntax sugar is for dictionaries. Here is an example, in which the key `"one"` is associated to the value `1`, and so on:
```
{ "one" :1, "two" :2, "three" :3 }
```
