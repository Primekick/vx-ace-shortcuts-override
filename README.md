# axer.tech | VX Ace Shortcuts Override

[![Static Badge](https://img.shields.io/badge/README-in_English-blue)](https://github.com/Primekick/vx-ace-shortcuts-override/blob/master/README.md)
[![Static Badge](https://img.shields.io/badge/README-po_polsku-red)](https://github.com/Primekick/vx-ace-shortcuts-override/blob/master/README_pl.md)

VX Ace Shortcuts Override, just like the name suggests, provides a way to bypass those wonky leftover keyboard shortcuts that get in they way of typing accented letters inside the RPG Maker VX Ace editor.

## How does it work?
This tool works by injecting a DLL file into currently running VXA editor instance. For this reason antivirus software will most probably prevent running it or even straight up remove it right after downloading, so to make it work you have to add the tool to the exceptions list.

## How do I use it?
![How do I use it](https://files.catbox.moe/3q79fi.webp)

- extract the zip at a desired location
- run vxa_shortcuts_override.exe
- editor's location should be automatically detected - if not, provide it manually
- a system notification will appear to indicate successful injection

## Features & roadmap
- [x] inject a DLL overriding nonsensical built-in shortcuts\
- [x] editor location detection
- [x] autolaunch with injection
- [ ] support for languages other than polish
- [ ] switch to a less invasive hotkey hooking if possible (i.e. not global)
- [ ] permanently patch the editor instead of having to run the fix manually with every lanuch

## Building
### Toolchain
Requires _i686-pc-windows-msvc_ target on _nightly_ channel.
