# netcrab:

#### A tool suite for creating, visualizing and analyzing Petri nets made in Rust

## Supported export formats:

- Petri Net Markup Language (PNML) [https://www.pnml.org/](https://www.pnml.org/)
- LoLA - A Low Level Petri Net Analyzer [https://theo.informatik.uni-rostock.de/theo-forschung/tools/lola/](https://theo.informatik.uni-rostock.de/theo-forschung/tools/lola/)
- DOT (graph description language) [https://en.wikipedia.org/wiki/DOT\_(graph_description_language)](<https://en.wikipedia.org/wiki/DOT_(graph_description_language)>)

## Petri net implementation

The main implementation is found in `net.rs`. It uses two `HashMap` to store the places and transitions. References to places and transitions are named `PlaceRef` and `TransitionRef` respectively. These types are a wrapper around `String`. Places and transitions are labelled with `String`. These must be unique in the net.

References to the places and transitions are returned when adding them to the net. These references can later be used to add arcs and to access the markings.

_Note: References can be cloned. One may have as many references to a node or transition as desired._

## Acknowledgments

Based on the original work from [https://github.com/Skasselbard/PetriToStar](https://github.com/Skasselbard/PetriToStar)
