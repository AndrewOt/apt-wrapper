# apt-wrapper

## The Problem
When you install a package with `apt`, _you_ are responsible for remembering what packages you had on your last build. So, when you switch debain distros or reinstall to start over, _you_ have to remember all the packages you installed which typically means you try to do something and it isn't there.

## The Solution
`apt-wrapper` tracks all of your installs and removes using a manifest in your home directory! That way, you don't have to remember all the packages you had before; make the computer do it all for you!

## Installation
WIP

## Usage
There are three simple commands for `apt-wrapper`
- `install` - installs packages and adds them to the manifest
- `remove` - removes packages the system and the manifest
- `restore` - installs all packages listed in the mainfest

The manifest resides in the top level of the home directory. Simply back it up with the rest of your home directory, install `apt-wrapper`, run `apt-wrapper restore` and you're good to go!
