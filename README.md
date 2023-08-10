# axer.tech | VX Ace Shortcuts Override

[![Static Badge](https://img.shields.io/badge/README-in_English-blue)](https://github.com/Primekick/vx-ace-shortcuts-override/blob/master/README.md)
[![Static Badge](https://img.shields.io/badge/README-po_polsku-red)](https://github.com/Primekick/vx-ace-shortcuts-override/blob/master/README_pl.md)

VX Ace Shortcuts Override, just like the name suggests, provides a way to bypass those wonky leftover keyboard shortcuts that get in they way of typing accented letters inside the RPG Maker VX Ace editor.

## How does it work?
This tool works by injecting a DLL file into currently running VXA editor instance. For this reason antivirus software will most probably prevent running it or even straight up remove it right after downloading, so to make it work you have to add the tool to the exceptions list.

## How do I use it?
https://github.com/Primekick/vx-ace-shortcuts-override/assets/48254804/c2473eb7-4ab9-4942-9e30-2aab982fff3b

- put both the at_vxa_so.dll and the exectuable in the same directory where the VX Ace editor resides
- launch the editor first
- run the vxa_shortcuts_override.exe
- if done correctly a popupwindow should appear to indicate the DLL was injected successfully 

## Features & roadmap
- [x] inject a DLL overriding nonsensical built-in shortcuts
- [ ] support for languages other than polish
- [ ] switch to a less invasive hotkey hooking if possible (i.e. not global)
- [ ] permanently patch the editor instead of having to run the fix manually with every lanuch

## Building
### Toolchain
Requires _i686-pc-windows-msvc_ target on _nightly_ channel.
