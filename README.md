# `gredit`

Gredit is a command-line application written in rust that allows you to quickly search in files, browse the context in which matches appear, and jump straight to matching lines in your default editor. 

It is still very early in development, here is a small demo:

![gif](/assets/demo.gif)

The grepping is done with [grep](https://docs.rs/grep/latest/grep/) crate, which is `ripgrep` as a library.

The TUI is built with [tui-rs](https://docs.rs/tui/0.10.0/tui/index.html), and the syntax highlighting is done with [syntect](https://docs.rs/syntect/latest/syntect/index.html), and the opening of the editor with a modified version of [edit](https://docs.rs/edit/latest/edit/index.html).

There is still a long way to go, and any contributions are greatly apprechiated!

