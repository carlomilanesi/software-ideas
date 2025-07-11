# Testing

## Hierarchy of tests

Tests should be automatic, as much as possible. This means that the codebase should contain code whose only purpose is to run tests.

Such tests can be started by persons or by other programs. When they are started by persons, the test runner displays clearly which tests have failed, if any. The test runner returns a success exit code when all the tests it has run were successful, and otherwise a failure exit code.

Tests should be in a hierarchy. The possible levels are: unit tests, integration tests, system functional tests, system performance tests.

First unit tests must be run. Only if all unit tests are successful, integration tests can be run, and so on up the hierarchy.

The tests of the same level should be run in *parallel*, to optimize performance, with a maximum number of concurrent tests. For example, the limits could be: at most 8 concurrent unit tests, 4 concurrent component tests, 4 concurrent integration tests, 2 concurrent system functional tests, 1 concurrent system performance test.

## Unit testing

**Unit tests** are the ones in which the application has no dependencies on other systems. Every access to the file system, to the network, to the current time, and to other processes must be mocked through dependency injection. So, system under test is a set of source code (one or more functions), not an executable module.

Unit tests should be written in the same programming language of the software under test.

## Integration testing

**Integration tests** are the ones in which the system under tests is a deployable executable file, i.e. an executable program or library. Its communication protocol is not mocked, but the communicating module are replaced by mock modules.

It can be written in any programming language capable of capturing the input/output of such executable file.

## System functional testing

**System tests** (sometimes named "end-to-end tests") are the ones in which all the systems under tests are deployed in a testing environment. The test runner does not capture the interactions among the modules of the system under test, but it captures the interactions between the system under test and the outside world, like the keyboard input, the screen output, and the file system.

In particular, **system functional tests** are meant just to check the correctness of the system, not its performance, and so the systenm is built with some runtime checks, and it is tested without heavy loads.

## System performance testing

**System performance tests** are similar to system functional tests, but the system was built with full optimization, it is run without optional runtime checks, and its performance is measured.

A particular case of a system performance test is a **system stress test**, in which the system is put under heavy load, to check what is the performance in such extreme conditions.

