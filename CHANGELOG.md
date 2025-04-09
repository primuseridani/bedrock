# Changelog

This is the changelog of Bedrock.
See `README.md` for more information.

## 0.5.0-5

* Add manual
* Update gitignore
* Update readme

## 0.5.0-4

* Add player tokens `Pawn`, `Bicycle`, `Train`, `Boot`, `Dog`, `Tank`, `Cannon`, `Duck`, and `Wastebasket`
* Add player teams
* Add `friendly_fire` flag to configurations
* Add `addr`, `name` and `password` fields to configuration
* Add base for network support
* Reconfigure release profile

## 0.5.0-3

* Change default level back to `lava_lake`
* Revert `f16` usage for shaders
* Remove Cargo lock from gitignore
* Remove `wgpu` as submodule
* Fix invalid veretx layout
* Update docs
* Rename `BERNIE` block tag to `COMBUSTIBLE`

## 0.5.0-2

* Add `is_spawnable` flag to chunks
* Update docs
* Fix test level
* Fix level parser expecting `layers` and not `layer`
* Add base for players
* Add `MissingSpawnChunk` error
* Temporarily add `wgpu` as submodule
* Fix lints
* Use `f16` in graphics
* Update lints

## 0.5.0-1

* Refactor level generator
* Support pausing
* Update pan controls

## 0.5.0-0

* Add layers to level configuration
* Update some maps
* Add docs for some facilities
* Support custom widths in chunks
* Rewrite level generator
* Rename `Stone` block to `Rock`

## 0.4.0

* Finalise version

## 0.4.0-7

* Enable Jemalloc

## 0.4.0-6

* Support CSS colours in level presets

## 0.4.0-5

* Update block colours
* Update background colour for *Lake* and *Lava Lake*
* Support setting level from cli
* Add controls overview
* Fix test level not specifying background
* Revert default level back to *Lake*

## 0.4.0-4

* Enable ticking
* Update block colours
* Add *Volatile* block tag
* Fix block tag testing
* Add *Lava Lake* level
* Fix viewport fragmentation
* Update zoom behaviour
* Update log formatting
* Rename *God* block tag to *Divine*
* Support adjusting tps

## 0.4.0-3

* Support view panning
* Update zoom controls
* Restructure code
* Add block tags
* Add more block materials
* Fix map generator
* Display map to window
* Update level format
* Embed random seed in blocks
* Update default map dimensions
* Update Valley level
* Add Lake level

## 0.4.0-2

* Support zooming with the (primary) mouse wheel
* Update logging

## 0.4.0-1

* Update graphics demo

## 0.4.0-0

* Update graphics demo
* Restructure code

## 0.3.1

* Add desktop entry
* Add readme
* Add installation script

## 0.3.0

* Finalise custom level loading
* Add basic graphics
* Set terminate handler

## 0.2.0

* Restructure code
* Support loading of custom levels
* Improve error handling
* Add more built-in levels
* Update level structure
* Add more block types
* Add logo

## 0.1.0

* Add configuration
* Generate levels
* Specify repository URL

## 0.0.0

* Add Cargo manifest
* Add changelog
* Add gitignore
