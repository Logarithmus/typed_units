### uom-ng

Design goals:
- Ability to express arbitrary combinations of units (via `typenum`)
- Units should be separate from values (like in C++'s `units`, unlike in `dimensioned`)
- `let speed = 10 * km / h` support (like `dimensioned`)
- Storage types, prefixed unit aliases & unit categories toggled via cargo features
- Lower case constants
- Nalgebra support
