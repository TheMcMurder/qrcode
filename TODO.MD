# Notes for development

## SVG Rendering
- Create an SVG group to contain the Finder nodes
- Create an SVG group to contain the data nodes
- Create an SVG group for layered filters
  - Example: A paintbrush effect where the nodes appear to be painted on ("paint lines"/filter could be generated randomly by using the URL as a seed in a randomizer)
  - Exaple: make the nodes clear showing a background gradient
- Consider rendering the finder nodes as a single unit instead of multiple blocks
- Consider rendering the data nodes as many grouped polygons instead of individual blocks (may be necessary for more complex shapes)

### Additional shapes to consider
- lines (barcode like) horizontal or vertical

## SVG -> PNG Rendering - we should prove this out sooner rather than later
- implement a way to convert SVG to PNG

## SVG -> JPEG Rendering - we should prove this out sooner rather than later
- implement a way to convert SVG to JPEG