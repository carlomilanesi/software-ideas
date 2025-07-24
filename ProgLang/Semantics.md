# Programming Language Semantics

## Assignment semantics

Consider this code, valid both in Python and in JavaScript:
```
a = [1, 2, 3]
b = a
b[1] = 7
```
After it, in both languages, and after similar code for other languages, the value of the variable `a` is `[1, 7, 3]`.

It means that the the second statement is just a reference assignment, creating an alias, and last line is a normal array write access, which changes a value shared by the variables `a` and `b`.
This behavior is named *alias semantics* or **share semantics**.

This semantics is not good for the understanding of the behavior of the code, because we are not explicitly changing the value of the variable `a`, but its value changes anyway.

What are the alternatives?

Before analyzing the performance of differtent kinds of assignments, let's consider that the objects assigned can be of two very different kinds:
* Shallow objects: Objects that do not contain references to other objects. To copy all the bits of such an object, in an operation named `shallow copy`, is enough to create a completely independent object. In addition, typically such objects are also rather small, from one byte, for a Boolean or a small enum, to a few tens of bytes, for a handful of 64-bit numbers.
* Deep objects: Objects that contain at least one reference to other objects. If all the bits of such an object are copied, the created object has references to the same objects referenced by the source objects, and so the copy shares such referenced object. To create completely independent objects, also the referenced objects must be copied to newly allocated objects, with an operation named `deep copy`. Such deep-copy allocations are quite costly. In addition, typically the referenced object can be quite large, even up to many megabytes, and so their copy can be very costly.

Here are the other possible semantics of assignment, in addition to share semantics:

* **Copy semantics**: The assignment creates a deep copy of the array, so that the final assignment changes only the value of the variable `b`, not the value of the variable `a`. This semantics is more understandable, but it can be more inefficient. In case it is used to pass a large data structure to a function which does not need to change it, it take much more time and much more memory than the copy of a reference.
* **Move semantics with initialization**: The assignment copies the reference to the destination (`b`), and it reinitialize the source (`a`). It is quite efficient, because just a reference is copied, but the value of `a` is changed even if the statement looks like just a copy.
* **Move semantics with invalidation**: The assignment copies the reference to the destination (`b`), and it makes the source (`a`) invalid: any attempt to use it afterwards is considered invalid by the compiler. This can be a little slower to compile and faster to run than the move semantics with initialization, because no initialization is needed, but it suffers the same unexpected behavior.
* **Immutable share semantics**: The assignment copies the reference to the destination (`b`), and it makes the destination immutable: any attempt to change the source or the destination afterwards is considered invalid by the compiler. This can have the same efficiency than the move semantics with invalidation, because no deep copy nor initialization are needed, but it causes the impossibility to change the values.
* **Copy-on-write semantics**: The assignment copies the reference to the destination (`b`), and marks both as shared: any attempt to change the source or the destination afterwards causes a deep copy of the structure. In case the object is shallow, like a number, no `shared` flag is needed, because a shallow copy is obviously enough. This semantics allows a very efficient assignment for shallow objects, a rather efficient assignment of deel objects, in case no change will be applied, and it allows also independent changes of the source and of the destination. Though, it is not as efficient as the previous solutions when frequent changes are applied, because every change of deep objects must check the `shared` flag.
* **Immutable share semantics and copy-on-write semantics of mutable objects**: The language requires that every variable is declared as mutable or as immutable. So, these cases of assignment are theoretically possible:
  * An immutable object is assigned to an immutable object: Immutable share semantics is applied.
  * An immutable object is assigned to a mutable object: This case is forbidden by the language, and so the compiler should generate a compilation error. This is because an immutable object shouldn't implicitly become mutable. If such a case is really needed, an explicit `clone` function should be called on the source object, to create a mutable copy to assign to the destination.
  * A mutable object is assigned to an immutable object: For the lifetime of the destination, the source becomes immutable and immutable share semantics is implemented; after the lifetime of the destination, the source becomes mutable again. In the typical case the assignment is performed as part of a function call, this is the usual behavior: the variables of the caller are not modified as long the called function is running.
  * A mutable object is assigned to a mutable object: Copy-on-write semantics is applied.

I think the last semantics is the bes one for application code: it is not much more difficult to undertand and use than typical application-oriented languages, and it allows only understandable behaviors, and it has reasonably good performance, as long as mutable objects are used only when really needed.
