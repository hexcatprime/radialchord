---
filename: README.md
title: README
author: dubsbol
date: 20240116 22:01:22
---
# Radial Chord
Game Controller Virtual Keyboard
## Impetus
I've had the misfortune of using a cursor to select keys on a virtual keyboard before. This happens on smart TVs and game systems, where a keyboard pops up on the screen and you're limited by the directional keys on a tv remote. **I hate it**.

typewriter-esque keyboards are not suited for all input methods. I had some experience setting up a microsoft hololens and was immediately frustrated with their AR keyboard. The hololens only tracks pointer fingers, which handicaps typing speeds on an AR keyboard. The hololens is better suited for tracking gross motor movements; I hope to bring this project into AR/VR environments too, should it reach maturation.

This project might also become an accessibility feature, useful to people who dont have fingers or full range of motion in their fingers. Ideally it would be adapted to be controller agnostic, accepting any input from something with two joysticks.

## Inspiration
This is predominantly a Rust programming exercise; it's my first experience with the language and I hope to learn enough to feel comfortable using it in the future.

My intended solution takes inspriation from the chording concept from stenographer's keyboards. Concurrent keystrokes are combined (chorded) to form complete syllables. In this implementation, the user would select a group of letters using the left joystick and choose which group letter to "print" with the right joystick.

The letter groupings are configurable, and based on english letter frequency right now which is largely unimportant. What poses to be the most interesting part of this project is input presentation and visual feedback. I'm applying the chording concept to radial menus found in console video games. Something like apex legend's ping wheel or fortnite's emote menu. Those menus are designed for joystick inputs, so it should feel natural for someone to type using two (hopefully).

The name Radial Chord comes from the combination of these two concepts, I may change it later but I chose it because it sounds cool.