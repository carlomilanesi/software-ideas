# Prototyping

Among the popular programming languages, there are some very efficient, like C language, and others very inefficient, like the operating system shells.

Among the popular programming styles, there are some very rigorous and strict, like Haskell or Rust, and others very indulgent and loose, like JavaScript.

Why are all these kinds of programming languages and programming styles popular? Which is better, and for which kinds of task?

A general rule is that more an error would be costly, more the application should be checked to avoid such an error.
For example, if an application is written to be used by a single person, any error would damage only that person, and so it would have a very small cost.
More people will be using that application, bigger will be the cost of an error.
In addition, some errors will appear only in rare conditions, and some others will appear every time.
Some errors will cause only minor inconvenience, like a wrong layout of a page, and some others will cause a major damage, like denial to use a system, or allowing unauthorized persons to access a critical system.

Another general rule is that using higher-level languages (with respect to Assembly language code) can introduce:
* lower development and maintenance costs;
* higher hardware costs to run the application;
* lower run-time performance, which has a cost.

Different application kinds and different conditions will cause different amounts for such costs.
The goal is to minimize the overall cost (development + hardware + wait-time + downtime + damages) for a time span of a few years.

A serious issue is the development of software with inappropriate requirements.
This is caused by this:
- Gathering detailed requirements is costly, and so it can be omitted. Though, the resulting product may be unsatisfactory for the users.
- Gathering appropriate requirements may be impossible, because even the users are not sure about which are the best requirements, until the product is used.
- Requirements specifications could be ambiguous.

A way to gather requirements is to build a prototype, that is a complete working application, written in a high-level language.
By using the prototype, users can specify whether the functional behavior is satisfactory or not.
A prototype is an unambiguous requirements specification.
A prototype is developed using a high-level language with interactive tools, so that changes are applied quickly.

An obvious question arises. If the prototype is already a complete working application, why another application is needed?

Actually a prototype has some drawbacks, with respect to a production application:
- It is slower (startup, throughput, latency). So, to use it properly, a costlier hardware is needed, and the application will have anyway an unsatisfactory usage.
- It needs more memory. So, to use it properly, a costlier hardware is needed, and the application will have anyway an unsatisfactory usage.
- It does not handle properly errors nor corner cases.
- It is not thoroughly tested, and so probably it has many undetected defects.
- It may be that the prototype will run only in a development environment, not in the target environment.

Sometimes, the following conditions are added:
- It is not modularized.
- It is not documented.
- Ease of maintenance is not taken into account.

Though, these last conditions are appropriate for the so-called proofs of concept, not for prototypes.

The main difference is that a proofs of concept is used only for the next version of the product.
After the product has been built, the proof of concept is to be discarded.
Therefore, proofs of concept are also named "throw-away prototypes".

Instead, a proper prototype must be reused for future versions of the product.
When the next version of the product is planned, first a new version of the prototype must be created, starting from the previous version.
Therefore, the source code of the prototype must be easily changeable.
After such changes to the prototype, a diff tool can show which are the changed portions of the codebase, and they can be translated into the production system code.
