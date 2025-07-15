# Versioning

To have a well-maintained code base, it is needed to follow these practices:
* Use a version control system (VCS) for all source code.
* Before every commit to the VCS, check which lines have been changed, to avoid committing unwanted changes.
* When committing to the VCS, write an explanation for the changed lines, to be added as commit message.
* Instead of committing directly to the main branch, commit to a topic branch.
* Create a pull request for a completed topic branch, having an explanation for the changes, obtained by summarizing the messages of the commits to that branch.
* Have at least one other person review the pull request, and all its changed lines, to approve or reject that pull request. Only approved pull requests can be merged into the main branch.

To perform the above tasks in the most effective and efficient way, it is needed to minimize the numer of bytes changed by any pull request. This can be obtained by following these practices:
* Use a standard format convention, so that no change will appear in a pull request just for changes in format. This can be performed using one of two possible tools, which should be automatically run on a document when it is saved:
  * A tool which changes the source code so that it will conform to the standard format (a formatter).
  * A tool which checks whether the standard format is used (a format checker, which highlights the invalid code).
* Use only line comments in committed code. Block comments should be used only to temporarily comment out some code.
* Have any list of things that are each one in a different line (statements, items of a collection, arguments of a function) have the same terminator (or no terminator), so that adding or removing one thing always changes only one line of code.
* Use only textual files in source code. In particular:
  * All documentation should be written in a textual format, like MarkDown, TeX, PlantUML, or similar. Avoid binary formats (like Microsoft Office files).
  * All resource files should have a textual source format, like SVG. If possible, avoid binary formats (like JPEG, PNG, MP3).
* Currently, my preferred document formats are Typst for general documents, and PlantUML for diagrams. They can be integrated by generating SVG files from PlantUML source, and embedding the generated SVG files into Typst documents. The best way to edit such documents is to use a split-screen editor; in a pane, the source code is edited, in the other pane that document is rendered. Avoid editing the rendered document.
