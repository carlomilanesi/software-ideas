# Versioning

To have a well-maintained code base, it is needed to follow these practices:
* Use a version control system (VCS) for all source code.
* Before every commit to the VCS, check which lines have been changed, to avoid committing unwanted changes.
* When committing to the VCS, write an explanation for the changed lines, to be added as commit message.
* Instead of committing directly to the main branch, commit to a branch.
* Create a pull request for a completed branch, having an explanation for the changes, obtained by summarizing the message of the commits to the branch.
* Have at least one other person review the pull request, and all its changed lines, to approve or reject that pull request.

To perform the above tasks in the most effective and efficient way, it is needed to minimize the numer of bytes changed by any pull request. This can be obtained by following these practices:
* Use a standard formatting convention, so thst no change will appear in a pull request just for formatting changes.
* Use only line comments in committed code. Block comments should be used only to temporarily comment out some code.
* Have any list of things that are each one in a different line (statements, items of a collection, arguments of a function) have the same terminator (or no terminator), so that adding or removing one thing always changes only one line of code.
