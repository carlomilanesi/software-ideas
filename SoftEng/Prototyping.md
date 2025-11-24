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
* lower development and maintenance costs
* higher hardware costs
* lower run-time performance, which has a cost

Different application kinds and different conditions cause different amounts for such costs.
The goal is to minimize the overall cost for a time span of a few years.
