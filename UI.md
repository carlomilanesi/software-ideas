# User Interface Principles

* In every instant, the UI must *show which commands are available*.
* There are two possible commands, the *exclusive* ones and the *concurrent* ones:
  * The **exclusive** commands block the application until such commands are completed, failed or aborted. When they run, the only available command is "Suspend", which tries to put the current exclusive command into a *suspended* state. When an exclusive command is in suspended state, the only available commands are "Resume" and "Abort". An example of an exclusive command is the command to send the current document to a printer spooler, because until the whole document has been sent to the spooler, the user shouldn't be able to change the document, or close the application.
  * The **concurrent** commands allow the application to receive other commands. Such other commands could automatically abort, and possibly restart the running concorrent command. An example of a concurrent command is the spellcheck of a natural-language text, or the syntax check of program code in an editor, because the user can apply changes even if such checks are not yet completed.
* Many developers believe that some commands are always quite fast, and other commands can last a long time. Actually, it is better to consider that every command can last a long time, because even commands that are usually quite fast can become very slow if the application or the whole system has become stucked or very slow for whatever reason.
* This is the algorithm to follow to handle every _exclusive_ command (because every command should be treated as possibly long):
  * The application should show immediately that it has received such a command.
  * After a short interval (say, 200 ms), if the command is not done yet, the application should display a "busy" icon (as the mouse cursor, or drawn in the window, or both). Such icon represents that some processing is under way.
  * After a longer period (say, 4 seconds), if the command is not done yet, the application should display a popup window containing:
    * A moving icon, representing that some processing is under way.
    * An estimate of the completion percentage, as a graphical progress bar, but also as the percentage value.
    * A possibly hierarchical description of the current stage.
    * An estimate of the remaining time, in hours, minutes, seconds.
    * A "Suspended" checkbox, to put the command in suspended state. It is initially in _unchecked_ state. When it is pressed, it becomes in _indeterminate_ state, until the command is actually suspended. When the command is actually suspended, that checkbox becomes in _checked_ state. When that button is pressed again, the command is resumed immediately, and the button becomes in _unckecked_ state.
    * An "Abort" button, enabled only when the "Suspended" checkbox is in _checked_ state. It aborts the _exclusive_ command.
* For every _concurrent_ command, the UI should should show whether the command is completed or not. For example, for a spellcheker, as soon as the check starts, the text to check is underlined in yellow, and when the check is completed, the text checked as good is no more underlined, and the text checked as bad is underlined in red.
* It should be possible to put every widget in one of these states:
  * **Enabled**. This means that it is shown normally, and anything can be done on it.
  * **Disabled**. This means that its contents cannot be changed, i.e. it is read-only. It can be operated to scroll through it, to copy its contents, to ask for help about it. It should be displayed in a different way to make clear that it is disabled, but it should be easily readable, and so, dark gray on light gray is not an acceptable color combination.
  * **Hidden**. It is not shown, but it is used by the layout manager.
  * **Collapsed**. It is not shown, and it is not used by the layout manager.
* The above one is a sequence of level, meaning that a collapsed widget behaves as also hidden and disabled, and a hidden widget behaves as also disabled.
* Some widget are containers, i.e. they can contain other widgets. For example, whole windows, tabs, group boxes, panes. When a container widget is enabled, any of the widgets it contains can be in different states (some enabled, some disabled, some hidden, some collapsed). Though, when a container widget is disabled, hidden or collapsed, all the widgets it contains should have no more capabilities than if they were, respectively, disabled, hidden or collapsed. For example, if an enabled widget contains two widgets, one enabled and one collapsed, and such a container is disabled, the first contained widget starts to behave as disabled and the other remains collapsed; when such a container is enabled again, the first contained widget re-starts to behave as enabled and the other remains collapsed.
* Every time a widget appears or is moved, such widget should be disabled for a short time (say, 300 ms), to avoid accidental commands. If the whole window appears or is moved, all its widgets should have such short disabling.
* Given that both the variables of application logic and the columns of most databases have several numeric types, in addition to the alphanumeric types, also the user interface should support such types, having a specific widget for inputting numeric values. Such numeric input widgets should:
  * Have a look similar to alphanumeric input widgets.
  * Display both the prompt inside the empty widget and the value as right-aligned.
  * Receive numeric values, instead of string values.
  * Return numeric values or string values, with different getters.
  * Use for keyboard input and for display the decimal separator specified by the current locale.
  * Automatically display thousands separators, according the current locale.
  * Have the optional numeric properties: `min_value`, `max_value`. Any editing operation exceeding such limits will have no effects.
  * Have the Boolean properties: `min_value_allowed`, `max_value_allowed`. Specifies whether the above limits are included or excluded by the allowed range.
  * Have the string properties: `prefix`, `suffix`. Such strings are displayed inside the box, but they are not editable.
  * Accept the clipboard Paste command, only if the clipboard content can be converted to a number included in the allowed range.
  * Accept the clipboard Cut/Copy commands, converting the widget content into a string.
* "Undo/Redo" Facility
  * The application should have "undo/redo" commands for internal commands, i.e. for every normal command changing only the internal state. This means that the commands explicitly changing also the state of other applications or of the file system can be not undoable. Though, logging and autosave should not prevent undoability.
  * Sometimes, it is useful to have "undo/redo" commands even for external commands, i.e. command explicitly changing the state of other applications or of the file system. In such a case, probably it is better to have separate "undo/redo" commands, with respect to the "undo/redo" commands for internal commands.
  * The undo/redo commands for the internal state should use the Command pattern to save in a memory structure the commands to undo and to redo a normal command. This usually requires much less space than saving a copy of the whole application state at every command.
* "Autosave" Facility
  * At every command, be it a normal commands, an _undo_ command, or a _redo_ command, the application should save locally the current state of the application. Such a saved state is overwritten at every command. When the application starts, it looks for a file containing such a state; if it is found, the last state of the application is restored.
  * To avoid slowing down the application, the _autosave_ operations are run in a separate thread, which is aborted by any user command. So, if, for example, the autosave operation takes 2 seconds, the autosaved state is available only 2 seconds after the last command.
  * To keep always at least one valid autosaved state, the _autosave_ operation should create a new temporary file, and, if such a file is successfully created, use it to replace the previous autosaved state.
  * To avoid discarding the last commands, the "exit" command must wait for the completion of the possible pending _autosave_ operation.
  * Applications could be run by fast typists, or by scripts which simulate user input at the fastest rate. To avoid continuously starting and aborting the _autosave_ operation, such operation should start only after a small delay (say, 80 ms).
 
# Common Defects of Interactive Applications

This is a list of frequent inadequate appearence (look) or behavior (feel) of the user-interface of software applications:
* **Regarding internationalization**
  * The application does not allow to set its current locale.
  * The application does not use the system locale as the default current locale.
  * The application has parts or all of its UI not internationalized or not properly localized.
* **Regarding debugging**
  * There is no unique way to determine the source code associated to a widget.
  * There is no unique way to determine the source code associated to a UI text.
  * There is no way or a cumbersome way to display a UI text in the language used by the developers, except by changing the application current locale.
  * It is too easy to display a UI text in the language used by the developers, so that users are bothered by such texts.
* **Regarding "undo/redo" facilities**
  * The application does not provide an "undo/redo" facility, or such facility is provided for only a single step, i.e. you cannot undo two connsecutive commands.
  * The application "undo/redo" commands do not correspond one-to-one to the normal commands, i.e. one "undo" command undoes several normal commands, or several "undo" commands are needed to undo a single normal command. As an example, some editors, when you press Enter to split a line, first move the rest of the line at the beginning of the new line, and then the new line is automatically indented. It is a single command, but the first "undo" command undoes only the automatic indentation, and another "undo" command is needed to undo the line splitting command. As another example, some editors, it you type a word of several characters, and then you execute the "undo" command, the whole word is removed.
  * The normal application commands, to be able to revert the application state to the previous value, always save the whole state of the application, which can be huge.
* **Regarding widgets to be clicked**
  * **Disguised buttons**: There is a text or an icon which is intended to be a button, but it does not look as a button, and so it is not clear whether users may click or tap on it.
  * **Unclear available commands**: It is not apparent whether, for a widget, there is an application behavior when it is pressed, single-clicked, double-clicked, or triple-clicked.
  * **Unclear behavior of commands**: There is a widget with a short text or an icon, and no way to know the assumed behavior of such a widget.
  * **No immediate feedback**: There is a widget which provides to the user no immediate feedback (visual, tactile, nor acoustic), when:
    * it is hovered;
    * it is pressed;
    * it is clicked (i.e. pressed and released inside of it).
  * **No debounce**: If a single-click on a widget has a behavior, but the user wrongly or accidentally does a double-click, the operation is performed twice, i.e. there is no debounce interval.
  * **Immediate enabling**: When a window or a widget is shown or it is moved, it is immediately active, and so the user can click on it when meaning to click on another widget that was in that position some milliseconds before. Instead, every widget should be disabled for a short time, when it appears or it is moved.
  * **Unclear disabling**: If a widget is disabled, it does not look as disabled.
  * **Unclear disabling reason**: If a widget is disabled, there is no way to know why it is disabled, and how to enable it.
* **Regarding long operations**
  * **No short-time processing feedback**. If an operation takes a noticeable amount of time (say, more than 200 ms), there is no way to know whether such an operation is still running or it is done.
  * **Regarding long-time processing feedback**: If an operation takes a long time (say, more than 4 seconds), there is no way to do this:
    * **No "stuck" feedback**: To know whether the current operation is proceeding or it is stuck forever.
    * **No current operation feedback**: To know which operation is under way.
    * **No current stage feedback**: To know which stage of the operation is under way.
    * **No advancement estimate**: To know an estimate of the percentage of completion of the operation.
    * **No time-to-run estimate**: To know an estimate of the remaining time for the completion of the operation.
    * **No pausing facility**: To pause this operation, and to resume it later.
    * **No aborting facility**: To abort this operation.
    * **No concurrent operation**: To perform other operations which can be run concurrently with this operation.
    * **Incompatible concurrent operation**: To avoid performing other operations which shouldn't be run concurrently with this operation.
* **Regarding input focus**
  * **Active window**: It is not enough clear which window is active, i.e. it has the input focus.
  * **Active widget**: It is not enough clear which widget of the active window is active, i.e. has the input focus.
  * **Active character**: It is not enough clear the current position or the current selection inside the active widget.
* **Regarding layout**
  * **Inappropriate widget size**: A text or an icon is very small, or very large, or two similar widgets have different sizes with no reason.
  * **Inappropriate contrast**: A text or an icon has too little contrast with respect to the background (like light gray on dark gray).
  * **Inappropriate internationalization**: The application is internationalized, and so the texts change in length when the language is changed, but the widgets do not move to make more room for longer texts or less room for shorter texts.
  * **Non-reactive layout**: The window is resizable, but the contained widget do not resize nor they change their layout. This is typical of applications designed for a specific screen resolution and run on a screen with a different resolution.
  * **Number alignment**: Some stacked numbers are left-aligned.
  * **Label-widget association**: A label and the corresponding widget are very far apart.
* **Regarding widgets meant to enter number**
  * It does not display the thousands separator according the system locale.
  * It does not display the fractional part separator according the system locale.
  * It display left-aligned values.
  * It allows to enter non-numeric characters.
  * It allows to enter numbers bigger than the maximum value.
  * It allows to enter numbers smaller than the minimum value.
  * If the minimumk value is positive or zero, it allows to enter the minus sign.
  * It allows to enter more fractional digits than the maximum value of fractional digits.
  * For a number representing a measure, no unit of measurement is displayed.
  * For a number representing an amount of money, no currency unit is displayed.  
* **Regarding validation**
  * Some simple kinds of data validation are not performed by the user-interface. The data is sent to the application logic, which returns an error message.
  * An error message is hard-to-understand, because it is addressed to developers.
  * An error message contains little information usefult for developers, because it is addressed to end-user.
  * Only the first validation error is displayed. So, the user must correct it, submit the data, just to find that there is another validation error. In case of many errors, this is frustrating.
  * The error messages cannot be copied into the clipboard.
  * An error message regarding a value does not show where is the invalid value.
* **Regarding concurrent validation**
  Usually, spell-checkers display possibly wrong words as underlined by a colored squiggle. A similar behavior is done by many programmers' editors, regarding possibly wrong portions of code. Usually, such checks start automatically as soon as the text is been changed. This feature may have the following defects:
  * **Undeterminate state**. Because such checks take some time, the previously wrong portions of text remains underlined as wrong, even if they are now correct, until the check is complete. Similarly, the previously correct portions of text remains not underlined as correct, even if they are now wrong. Instead, until checks are complete, changed values should look in an undetermined state (or "sub judice").
  * **High priority**. If the checks run at the same priority of the main UI thread, and they require many resources, the UI may be slowed down or even get stuck.
  * **Immediate check**. If the check start immediately after a change is done, there is the following drawback. Assume the keys are typed by a very fast typist, or by some automation software, at a rate of hundreds per second. So, in such a case, for hundreds of times per second the check is started and aborted. This slows down the application.

