# Loops

## The halting problem

For a programming language, is good or bad to be Turing-complete?

On one side, a programming language which is not Turing-complete is not as powerful as Turing-complete programming languages, and so its computational power is quite limited.

On another side, using a Turing-complete programming language never-ending programs can be written, and there is no way to write an algorithm capable of dectecting any never-ending program.
This is named the *halting problem*.

Instead, by using a programming language which gives up Turing-completeness, it is possible avoid never-ending programs.
This is useful not only because a never-ending program is probably wrong, but also because real-time systems must put bounds on the running-time and on the memory usage of their algorithms.
So, it is not possible to statically put upper bounds on the time and on the memory space which will be used by a program written using a Turing-complete programming language.
Instead, it may be possible to statically put such upper bounds, when using a non-Turing-complete programming language.

Every Turing-complete programming language must provide one or more of the following iteration statements:
* *recursive functions*;
* *backward `goto` statements;
* *potentially infinite loops*.

Let's call these kind of statements as "unbounded" statements.

Such constructs can be useful, but they are dangerous, because when they are used, a potentially infinitely-running program can be written.

Some languages, like C# and Rust, have the concept of *unsafe* code.
Code is by default *safe*, meaning that it can use ony a subset of the programming language, which is so restricted to avoid possible pitfalls.
When advanced techniques are needed, developers can define an *unsafe* section.

The same can be applied to "unbounded" statements.

In the safe portion of an application, unbounded statements should be avoided.
So, no recursion, which however is rarely used, except in functional programming, no *backward `goto`* statements, which however are banned from very most programming languages, and no *potentially infinite loops*.
The last kind of constructs includes, for the C language, `while` statements, `do-while` statements, and also `for` statements.

Does this mean that any kind of loop is banned?
No, the only kind of loops allowed are the ones over an finitely iterable object, like a finite range or a collection.
Any other kind of loop should be allowed only in an unsafe portion of code.
