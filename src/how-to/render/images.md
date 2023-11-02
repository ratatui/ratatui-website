# Rendering images

There's a few different ways to "draw images" in the terminal:

1. Sixel
2. Terminal specific control sequences (ITerm2 vs Kitty vs others)
3. Half block cells
4. Unicode Box drawing characters (link)
5. Unicode Line drawing characters (link)
6. Unicode characters for legacy computing (link)
7. Unicode Braille block (link)
8. Plain ASCII drawing

Sixel is a standard for drawing in the terminal but not all terminals support it.

The terminal specific control sequences are just that, terminal specific. And these are different
for each terminal, and there's no easy way to detect which terminal emulator you are running it, so
you have to choose ahead of time. But this allows you to draw _anything_. ITerm2 for example
supports drawing gifs to the terminal.

Both these two methods give high resolution images because it is the terminal emulator itself that
is drawing the images, as opposed to drawing text that is a lower fidelity representation of the
image.

The half block cells trick uses the fact that terminal fonts are typically 2 times taller than they
are wide. Here's a full block `█`. You can see a full block is 2 times taller than it is wide.
Here's a upper half block `▀` and lower half block `▄`. You can see that they only have foreground
text for either upper or lower halfs of a single "character".

Take into consideration just the lower half right now, i.e. `▄`. In terminals, you can color the
foreground and background for any individual `Cell` differently. By applying different colors to the
foreground and background for a upper half or lower half block, you get "twice the resolution" of
the image.

Braille characters have 8 dots per `Cell` that you can use to draw. This effectively makes the
resolution 8 times but you can't color dots differently, so it works well for black and white
images. See image attached:

https://en.wikipedia.org/wiki/Braille_Patterns?useskin=vector

There are other unicode characters can you can use to draw to the terminal. Check out this wiki page

https://en.wikipedia.org/wiki/Box-drawing_character?useskin=vector
