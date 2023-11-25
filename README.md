# Macroquad platformer example for Android

![Platformer screenshot](img/img1.jpg?raw=true "Platformer screenshot")

It has some junky touch support now
I build it in termux with custom cargo-quad-apk that is in my repos
For now I have local copy op macroquad, miniquad with commented lines in setFullScreen code in MainActivity because it doesn't work otherwise
Probably need to add custom option to cargo-quad-apk to use custom MainActivity or something, I don't know for now

Also I can make some kind of move/jump buttons or joystik instead of what I have now
And probably it would be better to lock camera on player and move with him and not fixed like I did now
