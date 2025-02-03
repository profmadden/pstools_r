# pstools_r
Rust version of pstools -- very lightweight PostScript file writing interface

=== 20241113 ===

Started hacking on this.  Moved the files around a bit in GitHub,
and making the structs.  Will have a vector of events to handle,
boxes, lines, text, color changes.  Will support layering, or maybe
do that in the future.  For short term, just generate PostScript.
Longer term, add in PNG support.

Text file commands:
* color r g b
* colora r g b
* fill
* nofill
* box llx lly urx ury
* line llx lly urx ury
* circle x y radius
* font fontname pointsize
* text x y Comments
* pscomment Comments

