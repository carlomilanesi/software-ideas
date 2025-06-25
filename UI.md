# User Interface Principles

* In every instant, the UI must show which commands are available.
* There are two possible commands:
  * The exclusive ones, which block the application until that command is completed, failed or aborted. When they run, the only available command is "Suspend". When an exclusive command it is in suspended state, the only available commands are "Resume" and "Abort". An example of an exclusive command is the command to send the current document to a printer spooler, because until the whole document has been sent to the spooler, the user shouldn't be able to change the document, or close the application.
  * The concurrent ones, which allow the application to receive other commands. Such other commands will automatically abort, and possibly restart the running concorrent command. An example of a concurrent command is the spellcheck of a natural-language text, or the syntax check of program code in an editor, because the user can apply changes even if such checks are not yet completed.
* Every command can last a long time, because even commands that are usually quite fast can become very slow if the application or the whole system has become stucked or very slow for whatever reason.
* For every possibly-long exclusive command (and so, for every command):
  * It should be displayed immediately that such a command has been received.
  * After a short interval (say, 200 ms), it should display an icon representing that some processing is under way.
  * After a longer period (say, 3 seconds), it should display a popup window containing:
    * A moving icon, representing that some processing is under way.
    * An estimate of the completion percentage.
    * A possibly hierarchical description of the current stage.
    * An estimate of the remaining time.
    * A two-state "Suspend" button, to put the command in suspended state.
    * A "Resume" button, enabled only in suspended state, to resume the command from where it was suspended.
    * An "Abort" button, enabled only in suspended state, to abort the command.
* For every concurrent command, the UI should should show whether the command is completed or not. For example, for a spellcheker, as soon as the check starts, the text to check is underlined in yellow, and when the check is completed the good text is no more underlined, and the bad text is underlined in red.
* Every widget which can be used in these states:
  * Enabled. This means that it is shown normally, and anything can be done on it.
  * Disabled. This means that its contents cannot be changed, i.e. is read-only. It can be operated to scroll through it, to copy its contents, to ask for help about it. It should be displayed in a different way to make clear that it is disabled, but it should be easily readable, so dark gray on light gray is not acceptable.
  * Visible. It is not shown, but is used by the layout manager.
  * Collapsed. It is not shown, and it is not used by the layout manager.
* Every time a widget appears or is moved, such widget should be disabled for a short time (say, 500 ms), to avoid accidental commands. If the whole window appears or is moved, all its widgets should have such short disabling.
