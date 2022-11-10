# Stalker

Stalker (official: stalker) is a simple file-watcher and custom executor with git-like syntax.

## Overview

Stalker will watch over user specified files and directories for changes in the files. Once stalker detect that there's a modification, stalker will then execute user specified action to the modified files only. Stalker will not do anything to the rest of the files, even if it's under the stalker supervision.

## Features

- Simple.
- Easy to use.
- Git-like syntax.
- Context-aware (It will not do anything to unrelated files, even if these files is under stalker supervision)
- Easy to integrate with other CLI applications.
- Runnable as a background process.

## Installation

To install stalker, you need to download the stalker executable from the *Releases* section on [stalker's GitHub page](https://github.com/PeterAjaaa/stalker).

Get the appropriate executable file for your corresponding OS.

***Important Note!***

**Linux** executables is the one ***without*** the extension (i.e. `stalk`).

**Windows** executables is the one ***with*** the extension (i.e. `stalk.exe`).

**Optional :**

Copy the stalker executables to /.cargo/bin so you can run stalker from anywhere in your machine.

(Note that the location of /.cargo/bin may differ from machine to machine, and from OS to OS.)

## Building from source

To build stalker, you need Rust to be installed in your machine.

If you have got Rust installed in your machine, follow these steps:

1. Do a `git clone` of the stalker's repository.

		git clone https://github.com/PeterAjaaa/stalker

	or for the stable release branch:

		git clone https://github.com/PeterAjaaa/stalker/tree/main

2. Enter the `stalker` directory.

		cd stalker

3. Use this command to build the `release` version of stalker:

		cargo build --release

4. The resulting executable will be located in `/target/release` directory under the executable name of `stalk` (note that depending on your OS, the executable will have different extension, e.g. stalk.exe for Windows).

## Usage

To get started with stalker, run this:

	stalk help

To get help for each individual subcommand, run:
	
	stalk <COMMAND> help

The usual workflow goes like this:

	stalk init
	stalk add path1 path2 path3 file1 file2 file3
	stalk do program-or-shell-command-to-run
	stalk execute

## Gotcha(s)

- ***DO NOT USE SHELL PATH EXPANSION WHEN SPECIFYING STALK DO COMMAND (E.G. THE USE OF ~ TO SPECIFY $HOME DIRECTORY, OR THE USE OF * TO SPECIFY ALL ITEMS IN A PATH). IT WILL NOT WORK!***

- ***DO NOT USE RELATIVE PATHS WHEN SPECIFYING STALK DO COMMAND (E.G. THE USE OF ../some-directory TO SPECIFY THE PARENT DIRECTORY RELATIVE TO CURRENT WORKING LOCATION). IT WILL NOT WORK!***

- ***USE ABSOLUTE PATHS WHEN SPECIFYING "STALK DO" COMMAND.***

## Todo

- ~~Cross-platform compability.~~
- Multiple stalker instances support.
- Removal of *expensive* operation where possible.
- Better file format to store action and paths for the stalker instance.
- Action removal from actionlist. [WORK IN PROGRESES]
