<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![Apache License][license-shield]][license-apache-url]

<div align="center">
<h1 align="center">Netcrab</h1>

## A simple library for creating and exporting Petri nets made in Rust

<img align="center" width="655" height="431" src="images/logo.png">

[Report Bug or Request a Feature](https://github.com/hlisdero/netcrab/issues)

</div>

## About The Project

### Petri net implementation

The main implementation is found in `petri_net.rs`. It uses two `BTreeMap` to store the places and transitions. References to places and transitions are named `PlaceRef` and `TransitionRef` respectively. Places and transitions are labelled with `String`. The net keeps the places in order, which allows the iterators to be deterministic.

References to the places and transitions are returned when adding them to the net. These references can later be used to add arcs and to access the markings.

_Note: References can be cloned. One may have as many references to a place or transition as desired._

### Supported export formats

- Petri Net Markup Language (PNML) [https://www.pnml.org/](https://www.pnml.org/)
- LoLA - A Low Level Petri Net Analyzer [https://theo.informatik.uni-rostock.de/theo-forschung/tools/lola/](https://theo.informatik.uni-rostock.de/theo-forschung/tools/lola/)
- DOT (graph description language) [https://en.wikipedia.org/wiki/DOT\_(graph_description_language)](<https://en.wikipedia.org/wiki/DOT_(graph_description_language)>)

### Built With

- [Rust](https://www.rust-lang.org/) > 1.65
- [cargo](https://doc.rust-lang.org/cargo/)
- [xml-rs](https://docs.rs/xml-rs/latest/xml/)

## Getting Started

To get a local copy up and running follow these simple example steps.

### Prerequisites

- Install Rust using one of the methods described on the [Rust Website](https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repo

   ```sh
   git clone https://github.com/hlisdero/netcrab.git
   ```

2. Build the project with `cargo`

   ```sh
   cargo build
   ```

3. Run the tests to check that everything works with `cargo`

   ```sh
   cargo test
   ```

## Usage

Creating a custom Petri net with a single place and a single transition forming a loop:

```rust
use netcrab::net::PetriNet;

let mut net = PetriNet::new();
let place_ref = net.add_place("Example place");
let transition_ref = net.add_transition("Example transition");

let result = net.add_arc_place_transition(&place_ref, &transition_ref);
assert!(result.is_ok());
let result = net.add_arc_transition_place(&transition_ref, &place_ref);
assert!(result.is_ok());
```

_Note: For more examples, please refer to the unit tests in each module._

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0). See [LICENSE-MIT](./LICENSE-MIT), [LICENSE-APACHE](./LICENSE-APACHE) for more information.

## Contact

Project Link: [https://github.com/hlisdero/netcrab](https://github.com/hlisdero/netcrab)

## Acknowledgments

Based on the original work by Tom Meyer found in <https://github.com/Skasselbard/PetriToStar>

This `README.md` is based on the template provided by [Best-README-Template](https://github.com/othneildrew/Best-README-Template)

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/hlisdero/netcrab.svg
[contributors-url]: https://github.com/hlisdero/netcrab/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/hlisdero/netcrab.svg
[forks-url]: https://github.com/hlisdero/netcrab/network/members
[stars-shield]: https://img.shields.io/github/stars/hlisdero/netcrab.svg
[stars-url]: https://github.com/hlisdero/netcrab/stargazers
[issues-shield]: https://img.shields.io/github/issues/hlisdero/netcrab.svg
[issues-url]: https://github.com/hlisdero/netcrab/issues
[license-shield]: https://img.shields.io/github/license/hlisdero/netcrab.svg
[license-apache-url]: https://github.com/hlisdero/netcrab/blob/master/LICENSE-APACHE
