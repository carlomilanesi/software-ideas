# Record-and-playback in software development

## Introduction

A very powerful family of techniques that can be used during software development is named "Record-and-playback".

It means that the software being developed can be run in three possibles modes:
* Recording: All (o some categories of) input/output operations of the application, in addition to communicating the data into the application (for input operations) or out from the application (for output operations), save the operation and its data into a specific log file.
* Playback: When the application starts, a log file, which was saved by the application during a previous session in recording mode, is read, and the logged data is used instead of the real data.
* Normal: This is the usual behavior, without reading nor writing any record-and-playback-specific log file.

## An example

Let's consider an example.

A CLI application prints to the console some requests to the user, who is expected to reply by entering some lines using the keyboard, and then such user input is used to access a database, and to print to the console the results obtained by the query.

For this application, there are several possible recording levels, as described by the following sections.

## Recording only user input

A possible recording mode writes to the log only the user input. The playback of that log does not expect any input from the user; it just performs all the prints to the console and all the accesses to the database.

The advantage of this technique is that the user input is saved into persistent storage, and so it can be repeated at will at maximum speed, with no waste of time for the user typing, and no risk of errors or oversights in user input. In addition, the results should be deterministic, as long as the database is not changed. Such a situation can be useful to debug the application, i.e. to inspect anomalies in correctness or performance.

## Recording also application output

Another possible recording mode writes to the log the user input, and also the console output, but not the database access. In case of success, the playback of that log has no visible output, but it can be used as a **regression integration test**, in the following way.

First, the correctly working application is run in recording mode, to collect a log of a use-case for correct behavior.

Then, the application can be run in playback mode using the generated log. The application gets its input from the log, and so there is no need for user input, and it gets two kinds of output: one from the log file, which is the expected output; and one from the application itself, which is the actual output. The application can check that such actual output is equal to the expected output, and it can emit an error message and stop the application in case of discrepancy.

Presumably, if neither the application logic nor the database have been changed, the playback-mode run should proceed to the end of the application.

Though, if the application logic has been changed, it is possible that the changed application will do a different operation sequence, or will emit a different output. In such a case, the playback of the log will find some discrepancy, this can be considered a test failure.

This kind of test is quite fragile, because there can be many possible changes to an application source code which will generate a different but correct console output, and so, this kind of testing can easily generate false positives (i.e. failing tests for correct code). For this, there are two possible solutions:
* Re-run the application in recording mode, to generate a correct log file.
* To edit manually the now-incorrect log file, to make it correct.

The first solution is better if the log file has changed a lot, or if its format is hard to understand for the tester.

The second solution is better if the log file has changed little, and its format is easy-enough to understand for the tester.

To have a log format which is easy-enough to understand for the tester, the following rules should be used:
* The testers should have some knowledge in computer technology, and not only in the application domain.
* The log file is a well-documented text file, easily readable both by humans and by computers.
* The log file is somewhat verbose, containing timestamps and some automatically generated comments, to make it more understandable.

Notice that this type of test is not a unit test, but an integration test, because the database is still accessed.

## Recording all input/output

Another possible recording mode writes to the log every input/output operation of the application: user input, console output, database requests, and database responses. The playback of that log performs no input/output operation (except the input operations from the log file), and so, it can be used as a **regression black-box unit test**.

It is similar to the previous case, but because in playback mode also the database is mocked, such mode does not depend on the existence and accessibility of a database having specific contents.
So, to run this application is playback mode is very fast and independent of the existence of other specific files or processes. This qualifies such procedure as a unit test.

Though, typically unit tests can test just portions of the application, by accessing them through source code. This is because they are a kind of *white box testing*, i.e. testing which depends on the source code of the application.

Instead, record-and-playback testing depends only on the external interface of the application, and so it is a kind of *black-box testing*.

## Static or dynamic implementation

To implement record-and-playback, it is needed to have three possible behaviors for every handled operation:
* Recording
* Playback
* Normal

This must be done by changing the source code of the application. Such kind of change is named *code instrumentation*.

There are at least two possible techniques to implement code instrumentation for record-and-playback:
* Static (or compile-time): A compiler option is set to specify the mode to use, and then, using compile-time conditional compilation, only the code regarding that mode is actually compiled.
* Dynamic (or run-time): An application option is set to specify the mode to use, and then, using run-time conditional statements, only the code regarding that mode is actually run.

In the first technique, a different executable file is generated by the compiler with different compiler options. This has the advantage to have maximum performance for normal mode. It has the following disadvantages:
* Three compilation commands are needed
* Three binary executables must be managed
* The in-editor code analyzer may be fooled, because its analysis should depends on the compiler option.

The file [Record-and-playback.rs](https://github.com/carlomilanesi/software-ideas/blob/main/SoftEng/Record-and-playback.rs) is an example of a static implementation in Rust language of this technique. At the beginning of the file, it is explained how to use it.
