# Introduction
"Manfred" is an over-engineered desktop pet. All you need is a sprite sheet and a simple config.

# Usage
when executed, the binary will try to load the config from a file called `config.yaml` from the current working directory. This is where you define the location of your sprite sheet, dimensions, animations and more. Right now, looking at the example cat config located in `example_configs` is probably the easiest way to get started.

# TODO
- ~~flipping animation frames~~
- ~~movement~~
- stop pet from walking off the screen & initial position
- actions
- loading zip archives
- basic CLI / custom path
- cursor interaction

# Notes
## Linux
On linux, depending on what window manager / compositor you use, you might have to add a few window rules.<br>
Here are the rules I use for Hyprland:
```conf
windowrule=noborder,title:^(Manfred)$
windowrule=xray off,title:^(Manfred)$
windowrule=noblur,title:^(Manfred)$
```