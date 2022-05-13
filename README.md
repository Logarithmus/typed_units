### uom-ng

Design goals:
- [x] Ability to express arbitrary combinations of units (via `typenum`)
- [x] Units should be separate from values (like in C++'s `units`, unlike in `dimensioned`)
- [x] `let speed = 10_f32 * (m / s)` support (like `dimensioned`)
- [ ] Storage types, prefixed unit aliases & unit categories toggled via cargo features
- [x] Lower case constants
- [ ] Nalgebra support
