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

Using choice 1, the expression `@f @g` means "first call `f` and then call `g`, passing the result of the first call as argument.
And the expression `f @g` means "call `g`, passing a reference to `f` as argument.

Using choice 2, the expression `f g` means "first call `f` and then call `g`, passing the result of the first call as argument.
And the expression `&f g` means "call `g`, passing a reference to `f` as argument.

Using choice 3, the expression `@f @g` means "first call `f` and then call `g`, passing the result of the first call as argument.
And the expression `&f @g` means "call `g`, passing a reference to `f` as argument.
