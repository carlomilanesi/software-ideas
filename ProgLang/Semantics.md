# Programming Language Semantics

## Assignment semantics

Consider this code, valid in both Python and JavaScript:
```
a = [1, 2, 3]
b = a
b[1] = 7
```
After it, for both languages, and after similar code for other programming languages, the value of the variable `a` is `[1, 7, 3]`.

It means that the second statement is just a reference assignment, creating an alias, and last line is a normal array write access, which changes a value shared by the variables `a` and `b`.
This behavior is named *alias semantics* or **share semantics**.

This semantics is not good for the understanding of the behavior of the code, because we are not explicitly changing the value of the variable `a`, but its value changes anyway.

What are the alternatives?

Before analyzing the performance of differtent kinds of assignments, let's consider that the objects assigned can be of two very different kinds:
* **Shallow objects**: Objects that do not contain references to other objects. To create a completely independent, equal object, it is enough to copy all the bits of the source object into the destination. Such an operation is named **shallow copy**. In addition, typically such objects are also rather small, from one byte, for a Boolean or a small enum, to a few tens of bytes, for a handful of 64-bit numbers.
* **Deep objects**: Objects that contain at least one reference to other objects. If just all the bits of such an object are copied, the destination object references will refer to the same objects referred by the source object, and so the copy shares such referenced objects. To create completely independent objects, also the referenced objects must be copied to newly allocated objects, with an operation named **deep copy**. Such deep-copy allocations are quite costly. In addition, typically the referenced object can be quite large, even up to many megabytes, and so their copy can be very costly.

Here are the other possible semantics of assignment, in addition to the share semantics:

* **Copy semantics**: The assignment creates a deep copy of the object, so that the final assignment changes only the value of the destination variable, not the value of the source variable. This semantics is more understandable, but it can be more inefficient. In case it is used to pass a large data structure to a function which does not need to change it, it takes much more time and much more memory than the copy of a reference, and so even languages not particularly concerned by performance, like Python or JavaScript, avoid it. Curiously, this is the default semantics of C++ assignments. Therefore, experienced C++ developers know that they must be careful when copying objects.
* **Move semantics with initialization**: The assignment makes a bitwise copy of the source to the destination, and it reinitialize the source. It is quite efficient, because just a short sequence of bytes is copied and another short sequence of bytes is zeroed. Though, the value of the source is changed to a default value, even if the statement looks like a copy. Actually, this is a destructive copy. This is C++ _move_ semantics.
* **Move semantics with invalidation**: The assignment makes a bitwise copy of the source to the destination, and it makes the source invalid: any attempt to use it afterwards is considered invalid by the compiler. With respect to the move semantics with initialization, this semantics can be a little slower to compile, because the compiler must keep track of which variables are valid. On the other side, it can be a little faster to run, because no initialization of the source is needed. Also this semantics can be somewhat misleading, because move when we were meant to copy the source, and instead we destroyed it. On the other side, no unexpected behavior can result from the use of the source, because the source object simply cannot be used anymore. This is the Rust assignment semantics for types which do not implement the trait `Copy`, like resizable strings and collections.
* **Immutable share semantics**: The assignment copies the reference to the destination, and it makes the destination immutable: any attempt to change the source or the destination afterwards is considered invalid by the compiler. This can have the same efficiency than the move semantics with invalidation, because no deep copy nor initialization are needed. Though, it causes the impossibility to change the values of both source and destination.
* **Copy-on-write semantics**: The assignment copies the reference to the destination, and marks both as shared at runtime: any further attempt to change the source or the destination causes a deep copy of the structure. In case the object is shallow, like a number, no `shared` flag is needed, because a shallow copy is obviously enough. This semantics allows a very efficient assignment for shallow objects, a rather efficient assignment of deep objects, in case no change will be applied, and it allows also independent changes of the source and of the destination. Though, it is not as efficient as the previous solutions when frequent changes are applied, because every change of deep objects must check the `shared` flag.
* **Immutable share semantics and copy-on-write semantics of mutable objects**: The language requires that every variable is declared as mutable or as immutable. So, these cases of assignment are theoretically possible:
  * An immutable object is assigned to an immutable object: Immutable share semantics is applied.
  * An immutable object is assigned to a mutable object: This case is forbidden by the language, and so the compiler should generate a compilation error. This is because an immutable object shouldn't implicitly become mutable. If such a case is really needed, an explicit `clone` function should be called on the source object, to create a mutable copy to assign to the destination.
  * A mutable object is assigned to an immutable object: For the lifetime of the destination, the source becomes immutable and immutable share semantics is implemented; after the lifetime of the destination, the source becomes mutable again. In the typical case the assignment is performed as part of a function call, this is the usual behavior: the variables of the caller are not modified as long the called function is running.
  * A mutable object is assigned to a mutable object: Copy-on-write semantics is applied.

I think the last semantics is the bes one for application code: it is not much more difficult to undertand and use than typical application-oriented languages, and it allows only understandable behaviors, and it has reasonably good performance, as long as mutable objects are used only when really needed.
