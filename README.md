# CATALYST

## About

A keybinding daemon for X11 that allows for more complex keybindings

## The Problem

I'm a big fan of Vim's modal mode, I think it's a great concept that allows for a great deal of creativity when it comes to making keybindings. But when it comes to system-wide keybindings, this isn't as prevelent, which I find really strange

## The Solution

As always, I'll just write my own.

This keybinding daemon is designed to be modal, but not in quite the same way as Vim. Instead, the modes depend on what state the desktop is currently in. If no windows are open, then a set of keybindings are available. If a window is open, another set is available. This makes it possible to have keybindings that otherwise overlap, for example browser keybindings, to be disabled when a window of that kind is open. This makes it possible to create key chords with more ergonomic keys, like the spacebar, and then have the spacebar still available when typing in a window.

## Roadmap

- [ ] Implement the basic keymapping system
- [ ] Make custom configuration available
- [ ] Something inbetween...
- [ ] Done :)
