# Chapters

This is the main guide which tracks the progress of the project. Each chapter
is designed as a standalone guide that focuses on a specific aspect of the
project.

The chapters are meant to be read in order, but you can jump to a specific
chapter if you are interested in a particular topic.

## Requirements

This guide assumes that you are running on a Windows machine with
[WSL](https://learn.microsoft.com/en-us/windows/wsl/install) and that you are
at least somewhat familiar with a Unix-like environment.

If you intend to follow along with a different setup, for example a Linux or
Mac OS machine, you may have to adapt some details. That being said, as most of
the work will be done on the Raspberry Pi, the differences should be minimal.

For the coding part, you may want to use a modern IDE like [Visual Studio
Code](https://code.visualstudio.com/). I personally use
[NeoVim](https://neovim.io/) but I won't go into the details of setting it up
here. This should not matter at all for the project.

## A note on commands

Sometimes, I will chose to show the expected output of a command instead of
just the command itself. This is to make it easier to understand what the
command does and what to expect from it.

When I do, the command will be prefixed with a `$` sign, like this:

```bash
$ echo "Hello, world!"
Hello, world!
```

This is to indicate that you should run the command `echo "Hello, world!"`
without the `$` prefix and that you should expect the output `Hello, world!`.

Some other times, I will show the command directly and without any prefix. This
allows for easier copy-pasting of the command:

```bash
sudo apt update
sudo apt upgrade
```

In any case, don't copy things blindly and take time to understand what each
command does and why it is needed.

## Glossary

- [Chapter 1 - Getting started!](./01-getting-started/README.md)
