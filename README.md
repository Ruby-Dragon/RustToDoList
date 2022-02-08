# RustToDoList (WIP)

###### GNU Public Licence v3, 2022, Ruby-Dragon

to do list in Rust.

This is how I will learn Rust. Seems like a cool
language. I like lists apparently.

make tasks, remove tasks, and complete tasks.

Note: Only Windows Terminal or a Linux terminal such as Kitty, Konsole, or Gnome Terminal will show completed tasks with a strikethrough. Any other console will not display anything to indicate completeness!

It is recommended that Windows users use WSL2 to compile and run this program using Windows Terminal and set bash aliases within the WSL installation.

## Usage:

	There is one list that is maintained throught usage of the program.
	
	Add tasks using the add command, which takes two parameters. The first one is the task name/description. This cannot have any whitespace.
	The second parameter is the date that the task should be completed. Date is in this format, yyyy-m-dd.

	Example of add: "add complete-program 2022-2-14" - adds task complete-program with a due date of 2022-2-14.


	Remove tasks with the rm command, which takes one parameter. This parameter is the number of the task to be removed.

	Example of rm: "rm 2" - removes task at number 2.


	Complete tasks with comp command, which takes one parameter. This parameter is the number of the task to be completed.

	Example of comp: "comp 2" - completes task at number 2.

	Completed tasks will be crossed out until their due date, where they will be automatically removed.

## Installation:

	To install, first install rust, and make sure you install cargo with it.

	Download the source code from github, or use git to clone the repo.

	"git clone https://github.com/Ruby-Dragon/RustToDoList.git"

	Open a command line window in the rust_to_do_list directory in the downloaded source code.
	Run this command in the command line windows:

	"cargo build --release"

	Cargo will the compile the code into an executable file.

	Move the executable from target/release to wherever you want to keep it.
	To run, use "./rust_to_do_list" in a command line window.

	If you use bash, you may want to add an alias to make it easier to use!

	This process is cross platform and should work on linux, windows, and OSX.

## Licence:

This software uses the GPL licence. Read the terms before using the source code.

###### ALL DERIVATIVE WORKS MUST BE GPL v3 LICENCED AS WELL, AND MUST KEEP ALL COPYRIGHT NOTICES IN CODE. See the LICENCE for more information.
