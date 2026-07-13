# Changelog

## plotters-dixous 1.0.0

### Breaking
- api is now divided into svg and bitmap backends
- svg plotter is now behind the svg feature flag (enabled by default)
- bitmap plotter is now behind the bitmap feature flag (enabled by default)
- plotting elements now uses the dioxus v7 component system

### Dependencies
- updated image to 0.25.10
- updated base64 to 0.22.1
- updated dioxus to 0.7.9
- updated plotters 0.3.7

### Added
- Support for plotter's `SVGBackend`

## plotters-dixous 0.2.2

### Added

- add the complete list of drag events on the component *plotters**

## plotters-dixous 0.2.1

### Added

- add the complete list of mouse event on the component *plotters*

## plotters-dioxus 0.2.0 (2024-02-22)

### Added

- callback to handle the on_wheel and on_click action.

### Improved

- Change the rendering system because of performance issue: 
  - does not use svg anymore
  - Draw directly in a bitmap
  - render the bitmap in the html with a base64 encoding.
- Rename the user interface *on_drawing* into *init*

### Removed

- The *dioxus* backend which generate *LazyNodes*

## plotters-dioxus 0.1.0 (2024-02-18)

### Added

- A *dioxus* backend implementing the trait *DrawingBackend* from *plotter-rs* to generate *LazyNodes of svg items*
- A *dioxus* component named **Plotter**, using the *dioxus* backend to render the plots.
- A *callback* on_drawing for the component **Plotter** as user interface to define plots. 
