# Syntax of expressions

## Use postfix notation

Very most programming languages use infix binary operators, like in the expression `a + b`, and prefix function call, like in `f(x)` or `(f x)` or simply `f x`.

The only common exception is for method calls, like in `a.f(b)` in which the function name is put after the first argument and before all the other arguments.

The disadvantage of the infix and the prefix notations is that if the last expression is not a simple literal, like `3`, or a simple identifier, like `a`, the eye must first encounter the name of the function and much later that of its operand, which must be evaluated before calling the function. The reading order is not the same of the evaluation order.

Instead, it is much easier to read code in which the reading order is the same of the evaluation order. This happens only in postfix notation. So the following expressions are better: `a b +` instead of `a + b`, `and `x y f` instead of `f(x, y)` or `x.f(y)`.

In addition, many languages have complex precedence rules for infix operators. Such rules which can be overridden by using parentheses. Such rules are usually so complex to learn and so error prone, that many exprerts suggest to use parentheses anyway, to make clearer the precedence. Instead, postfix notation, like prefix notation, do not require any precedence rules nor parentheses to force precedence.

## Functions calls vs function references

In every non-trivial programming language, a function can receive as argument a reference to a function. So, whenever the name of a function with no arguments is encountered, it is needed some syntax to distinguish the case in which we want to call that function from the case in which we want to pass a reference to that function.

There are three reasonable choices:
1. To use just the name of the function to specify a call, and a decoration to specify a reference.
2. To use just the name of the function to specify a reference, and a decoration to specify a call.
3. To use a decoration of the name of the function to specify a reference, and a different decoration to specify a call.

For example, let's assume we have a function named `f` with no arguments, and a function named `g` receiving as argument a reference to function.
In addition, assume the symbol `@` prefixed to a function name means that we want to call that function, and symbol `&` prefixed to a function name means that we want to get a reference to that function.

Using choice 1, the expression `f g` means "first call `f`, and then call `g`, passing as argument the result of call to `f`.
And the expression `&f g` means "call `g`, passing as argument a reference to `f`.

Using choice 2, the expression `@f @g` means "first call `f`, and then call `g`, passing as argument the result of the call to `f`.
And the expression `f @g` means "call `g`, passing as argument a reference to `f`.

Using choice 3, the expression `@f @g` means "first call `f`, and then call `g`, passing as argument the result of the call to `f`.
And the expression `&f @g` means "call `g`, passing as argument a reference to `f`.

What should be considered is this:
* Conceptually, calling a function is something more complex than taking the reference to that function, and so it deserve a more complex notation, like in choice 2.
* In typical applications, calls to functions are much more common than getting references to functions, and so choice 1 is a more concise notation.
* In many languages, the decoration to specify a function call is just the pair of parentheses enclosing the possible arguments. Such parentheses are anyway useful for readability, to make clear where the list of arguments ends (for prefix notation) or begins (for postfix notation).

## Specifying argument names at function calls

In most programming languages, this is an allowed syntax to call a function: `f(12, 7)`. That function could be defined to have the first argument with name `x`, and the second argument with name `b`. At the call statement, such argument names could be displayed by the editor in a tooltip, but they are not shown in source code. Some languages allow to optionally specify the names of the arguments.

Having function calls which display the names of the arguments is a big readability improvement, like when a structure is constructed by specifying the names of the fields, instead of just specifying a list of values.

## Function call syntax

Considering what is written above, I think that a good syntax is the following one.

There is a syntax to specify the instance of a structure like this one: `{ a: 12, b: 7 }`. This structure can be assigned to a variable of the appropriate type.

In addition structure instance specification can be followed by the name of a function, like in this expression: `{ a: 12, b: 7 } f`. This expression is a call to the function `f`, passing as arguments `12` for the argument `a`, and `7` for the argument `b`.

Consider instead this expression: `{ arg: f } g`. This is a call to the function `g`, passing to it one argument, named `arg`, and having a reference to the `f` function as value.

Consider instead this other expression: `{ arg: {} f } g`. To evaluate this expression, first the `f` function is called, with no arguments, and then the function `g` is called, passing to it the result of the call to `f`.
