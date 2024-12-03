### 20241203

Added in support for defining a bounding box, or adding a border.
Also, can toggle on/off fill now.  Bounding boxes should be reasonable
now (in the EPS file its lower-left/upper right, not origin+width-and-height).

Might need to tweak the way fill is handled; a bit of a kludge.

Adding text -- can't have text as part of the union, so it's not part of
the event list.  Planning on having a second vector containing the text
events, and these will get overlaid on top, after all the other drawing.
Maybe not ideal, but it should work.

Text will have an XY coord, a font size, and the actual text to display.

----

Rethinking.  Have a text event in the union, and for strings, it will
reference a string number in a separate array.  Can also have a setfont
operation (scale of the font, plus typeface).  The secondary array
of strings will have the actual string, while the event in the event
list has a string list index.

This lets me interweave text and graphics, change font sizes and colors,
and it should all more-or-less work.

Still kind of a kludge, but will likely be very helpful.

----

Reworked, and I'm liking it better.  For the box, include a closepath
before the stroke.

Should add support for curveto (move to XY, x1y1 x2y2 x3y3 curveto stroke).

And maybe a way to directly inject some sort of PostScript sequence?  Might
as well.

Plus, set the line width, and a few other things.  But it's looking a lot
more legit now.

