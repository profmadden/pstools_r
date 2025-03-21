# pstools_r
Rust version of pstools -- very lightweight PostScript file writing interface

The crate can be used as a callable library; it can also build a stand-alone tool that
parses inputs with simple commands.

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

