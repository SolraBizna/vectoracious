A V2D file is a sequence of commands. Each command is a command name followed by zero or more parameters, specified by spaces. There is no provision for escaping. Any portion of a line following a `#` is discarded. Empty lines are ignored.

# Commands

## `c` - set color

Valid forms:

- `c r g b` or `c r g b a`: each color component is given separately. if a period is present in the component, it's an value from 0 to 1 inclusive. if it's not, it's a value from 0 to 255 inclusive. color values are sRGB, alpha value is linear.
- `c rgb` or `c rgba` or `c rrggbb` or `c rrggbbaa`: HTML-ish hex notation. color values are sRGB, alpha value is linear.
- `pc ...` is the same as `c ...` but the given color must already have been pre-multiplied
- `lc ...` is the same as `c ...` but all components are linear
- `lpc ...` and `plc ...` combine the special meanings of `pc` and `lc`

The first color specified is index 0, the second color is index 1, and so forth. Up to 256 colors may be specified. The renderer can override colors, and does so by index. By convention, a given application of color override overrides the lowest-numbered indices, so e.g. 0 might be a team banner color or an explosion state, 1 might be a team hilight...

## `bb in_x in_y in_w in_h out_x out_y out_w out_h` - set bounding box

Specifies the bounding box used to map subsequent `p` commands.

Example:

```
bb 0 0 64 64 -1 -1 2 2
```

In this example, input coordinates in the range +0 to +64 will be mapped to coordinates in the range -1 to +1.

When a bounding box isn't specified, it's as though `bb 0 0 1 1 0 0 1 1` were used (i.e. coordinates aren't altered)

## `p colorid x1 y1 x2 y2 x3 y3 [x4 y4 ...]` - polygon

Specifies a polygon of the given color.

