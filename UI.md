# User Interface Principles

* In every instant, the UI must *show which commands are available*.
* There are two possible commands, the *exclusive* ones and the *concurrent* ones:
  * The **exclusive** commands block the application until such commands are completed, failed or aborted. When they run, the only available command is "Suspend", which tries to put the current exclusive command into a *suspended* state. When an exclusive command is in suspended state, the only available commands are "Resume" and "Abort". An example of an exclusive command is the command to send the current document to a printer spooler, because until the whole document has been sent to the spooler, the user shouldn't be able to change the document, or close the application.
  * The **concurrent** commands allow the application to receive other commands. Such other commands could automatically abort, and possibly restart the running concorrent command. An example of a concurrent command is the spellcheck of a natural-language text, or the syntax check of program code in an editor, because the user can apply changes even if such checks are not yet completed.
* Many developer believe that some commands are always quite fast, and other commands can last a long time. Actually, it is better to consider that every command can last a long time, because even commands that are usually quite fast can become very slow if the application or the whole system has become stucked or very slow for whatever reason.
* To handle the case of a possibly-long exclusive command, and so, for every exclusive command:
  * The application should show immediately that it has received such a command.
  * After a short interval (say, 200 ms), if the command is not done yet, the application should display an icon representing that some processing is under way.
  * After a longer period (say, 3 seconds), if the command is not done yet, the application should display a popup window containing:
    * A moving icon, representing that some processing is under way.
    * An estimate of the completion percentage, as a progress bar, containing the percentage value.
    * A possibly hierarchical description of the current stage.
    * An estimate of the remaining time, in hours, minutes, seconds.
    * A "Suspended" checkbox, to put the command in suspended state. It is initially in unchecked state. When it is pressed, it becomes in indeterminate state, until the command is actually suspended. When the command is actually suspended, that checkbox becomes in checked state. When that button is pressed again, the command is resumed immediately, and the button becomes in unckecked state.
    * An "Abort" button, enabled only when the "Suspended" checkbox is in checked state. It aborts the exclusive command.
* For every concurrent command, the UI should should show whether the command is completed or not. For example, for a spellcheker, as soon as the check starts, the text to check is underlined in yellow, and when the check is completed the good text is no more underlined, and the bad text is underlined in red.
* Every widget which can be used in these states:
  * Enabled. This means that it is shown normally, and anything can be done on it.
  * Disabled. This means that its contents cannot be changed, i.e. is read-only. It can be operated to scroll through it, to copy its contents, to ask for help about it. It should be displayed in a different way to make clear that it is disabled, but it should be easily readable, so dark gray on light gray is not acceptable.
  * Visible. It is not shown, but is used by the layout manager.
  * Collapsed. It is not shown, and it is not used by the layout manager.
* Every time a widget appears or is moved, such widget should be disabled for a short time (say, 500 ms), to avoid accidental commands. If the whole window appears or is moved, all its widgets should have such short disabling.
* Given that both the application logic and most databases have variable of numeric types, in addition to the alphanumeric types, also the user interface should support such type, having a specific widget for inputting numeric values. Such numeric input widgets should:
  * Have a look similar to alphanumeric input widgets.
  * Display both the prompt and the value right-aligned.
  * Receive and return numeric values, instead of string values.
  * Automatically display thousands separators.
  * Have the optional numeric properties: `min_value`, `max_value`. Any editing operation exceeding such limits will have no effects.
  * Have the Boolean properties: `min_value_allowed`, `max_value_allowed`. Specifies whether the above limits are included or excluded by the allowed range.
  * Have the string properties: `prefix`, `suffix`. Such strings are displayed inside the box, but they are not editable.
  * Accept the clipboard Paste command, only if the clipboard content can be converted to a number included in the allowed range.
  * Accept the clipboard Cut/Copy commands, converting the widget content into a string.
* "Undo/Redo" Facility
  * The application should have "undo/redo" commands for internal commands, i.e. for every normal command changing only the internal state. This means that the commands explicitly changing also the state of other applications or of the file system can be not undoable. Though, logging and autosave are not a good reason to avoid undoability.
  * Sometimes, it is useful to have "undo/redo" commands even for external commands, i.e. command explicitly changing the state of other applications or of the file system. In such a case, probably it is better to have separate "undo/redo" commands, with respect to the "undo/redo" commands for internal commands.
  * The undo/redo commands for the internal state should use the Command pattern to save in a memory structure the commands to undo and to redo a normal command. This usually requires much less space than saving a copy of the whole application state at every command.
* "Autosave" Facility
  * At every command, be them normal commands, "undo" commands, or "redo" commands, the application should save locally the current state of the application. Such a saved state is overwritten at every command. When the application starts, it looks for an autosaved state; if it is found, its last state is restored.
  * To avoid slowing down the application, the autosave commands are run in a separate thread, which is aborted by any command. So, if, for example, the autosave command takes 2 seconds, the autosaved state is available only 2 seconds after the last command. To avoid discarding the last commands, the "exit" command must wait for the completion of the autosave command.
  * Application could be run by fast typists, or by scripts which simulate user input at the fastest rate. To avoid continuously starting and aborting the autosave command, such autosave should start only after a small delay (say, 80 ms).
  * To keep always at least one valid autosaved state, the autosave feature should create a new temporary file, and, if such a file is successfully created, use it to replace the previous autosaved state.
 
# Common Defects of Interactive Applications

This is a list of frequent inadequate appearence (look) or behavior (feel) of the user-interface of software applications:
* Regarding "undo/redo" facilities
  * The application does not provide an "undo/redo" facility, or such facility is provided for only a single step, i.e. you cannot undo two connsecutive commands.
  * The application "undo/redo" commands do not correspond one-to-one to the normal commands, i.e. one "undo" command undoes several normal commands, or several "undo" commands are needed to undo a single normal command. As an example, some editors, when you press Enter to split a line, first move the rest of the line at the beginning of the new line, and then the new line is automatically indented. It is a single command, but the first "undo" command undoes only the automatic indentation, and another "undo" command is needed to undo the line splitting command. As another example, some editors, it you type a word of several characters, and then you execute the "undo" command, the whole word is removed.
  * The normal application commands, to be able to revert the application state to the previous value, always save the whole state of the application, which can be huge.
* Regarding widgets to be clicked
  * There is a text or an icon which is intended to be a button, but it does not look as a button, and so it is not clear whether users may click or tap on it.
  * It is not apparent whether, for a widget, there is an application behavior when it is pressed, single-clicked, double-clicked, or triple-clicked.
  * There is a widget with a short text or an icon, and no way to know the assumed behavior of such a widget.
  * There is a widget which provides to the user no immediate feedback (visual, tactile, nor acoustic), when:
    * it is hovered;
    * it is pressed;
    * it is clicked (i.e. pressed and released inside of it).
  * If a single-click on a widget has a behavior, but the user wrongly or accidentally does a double-click, the operation is performed twice, i.e. there is no debounce interval.
  * When a window or a widget is shown or is moved, it is immediately active, and so the user can click on it when meaning to click on another widget that was in that position some milliseconds before.
  * If a widget is disabled, it does not look as disabled.
  * If a widget is disabled, there is no way to know why it is disabled, and how to enable it.
* Regarding long operation
  * If an operation takes some time (say, more than 200 ms), there is no way to know whether such an operation is still running or it is done.
  * If an operation takes a long time (say, more than 4 seconds), there is no way:
    * to know which operation is under way;
    * to know which stage of the operation is under way;
    * to know an estimate of the percentage of completion of the operation;
    * to know an estimate of the remaining time for the completion of the operation;
    * to pause this operation, and to resume it later;
    * to abort this operation;
    * to perform other operations which can be run concurrently with this operation;
    * to avoid performing other operations which shouldn't be run concurrently with this operation.
* Regarding input focus
  * It is not enough clear which window is active, i.e. it has the input focus.
  * It is not enough clear which widget of the active window has the input focus.
* Regarding layout
  * A text or an icon is very small, or very large, or two similar widgets have different sizes with no reason.
  * A text or an icon has too little contrast with respect to the background (like light gray on dark gray).
  * The application is internationalized, and so the texts change in length when the language is changed, but the widgets do not move to make more room for longer texts or less room for shorter texts.
  * The window is resizable, but the contained widget do not resize nor they change their layout. This is typical of applications designed for a specific screen resolution and run on a screen with a different resolution.
  * Some stacked numbers are left-aligned.
  * A label and the corresponding widget are very far apart.
* Regarding numbers
  * A number representing a measure displays no unit of measurement.
  * A number representing an amount of money displays no currency unit.
  * A widget meant to enter a number allows to enter non-numeric characters.
* Regarding validation
  * Some simple kinds of data validation are not performed by the user-interface. The data is sent to the application logic, which fails with a hard-to-understand error message.
  * Only the first validation error is displayed. So, the user must correct it, submit the data, just to find that there is another validation error. In case of many errors, this is frustrating.
  * The error messages cannot be copied into the clipboard.
  * The error messages do not show where is the invalid value.
  * Usually, spell-checkers display possibly wrong words as underlined by a colored squiggle. Similarly, many programmers editors display possibly wrong portions of code as underlined by a colored squiggle. Usually, such checks starts automatically while the text is changed. Because such checks takes some time, until the check is complete, the previously wrong portions of text remains underlined as wrong, even if they are now correct, and the previously correct portions of text remains not underlined as correct, even if they are now wrong. Instead, they should look in an undetermined state.
