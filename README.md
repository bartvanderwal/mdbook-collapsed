# mdbook-collapsed

Important Note: This plugin does NOT work yet. I'm writing it for my own use on the website [minordevops.nl](https://minordevops.nl) for the minor I teach. But I'm a complete [Rust](https://www.rust-lang.org/) newbie. I'm getting a lot of help of ChatGPT as well. Thought it's NOT as good in Rust as it is in other languages. It's also a good forray into the world of Prompt Engineering and the current LLM's programming capabilities.

A preprocessor plugin for [mdBook](https://rust-lang.github.io/mdBook/) that adds collapsible sections to your book's content (initially collapsed by default). A collapsible section is defined by adding `#collapsed` at the end of any heading, making it easier to create interactive and cleaner documentation.

Notice the name is 'collapsed' not 'collapsible'. This is beacuse by default the sections are collapsed. This is intentional, to have a less busy page by default.

Important: This plugin only supports collapsing sections in `#` headers. It does NOT (yet) support [Setext](https://en.wikipedia.org/wiki/Setext) style headers, e.g. headers which look like this:

```markdown
Header 1
========

Header 2
--------
```

This might be added later. Or add a pull request, once it's stable.

## How to Run

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (for building the plugin)
- [mdBook](https://github.com/rust-lang/mdBook) installed

### Installation

1. Clone this repository:

```bash
git clone https://github.com/your-username/mdbook-collapsed
cd mdbook-collapsed
```

2. Build the plugin:

```bash
cargo build --release
```

3. Optionally, move the binary to a location in your `$PATH`:

```bash
sudo cp target/release/mdbook-collapsed /usr/local/bin/
```

## Usage

In your mdBook project, open book.toml and add the following lines to configure the plugin:

```toml
[preprocessor.collapsed-preprocessor]
```

In your Markdown files, define collapsible sections by adding #collapsed to any heading. For example:

```markdown
## Example Section #collapsed

This content will be hidden by default and can be expanded by clicking the heading.
```

Build your mdBook:

```bash
mdbook build
```

Optionally, serve the book for local viewing:

```bash
mdbook serve
```

You can now view the collapsible sections in your browser at `http://localhost:3000`.

## Examples

Here's how you can add collapsible sections in your Markdown:

```markdown
# Introduction #collapsed

This is the introduction content.

## Details #collapsed

More detailed content that can be hidden.
This will render as headings that can be clicked to expand or collapse the content beneath them.
```

## Features

Automatically collapses any section with a heading ending in #collapsed.
Simple, intuitive configuration through Markdown syntax.
Retains users' progress (checked items, expanded sections) using `localStorage`.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests. Please follow the coding standards and test thoroughly. ## Contributing

We welcome contributions to `mdbook-collapsed`! If you're interested in contributing, please refer to the [CONTRIBUTING.md](./CONTRIBUTING.md) file for detailed guidelines on how to get started, including coding standards and submitting pull requests.

### Quick Start for Contributors

Here’s a short overview of how to compile and build the project:

1. **Clone the repository**:

```bash
git clone https://github.com/<your-username>/mdbook-collapsed.git
cd mdbook-collapsed
```

Build the plugin:

```bash
cargo build --release
```

The compiled binary will be available in the target/release/ directory.

Test the plugin in an mdBook project by adding the plugin to the `book.toml` file adding a `#collapsed` hashtag on a test header somewhere and (re)building the book like described above to see if it works.

For more details on testing, code style, and submitting changes, check out CONTRIBUTING.md.

## License

This project is licensed under the [MIT License](https://opensource.org/license/mit).

## Note on the use of ChatGPT

ChatGPT did also write large parts of this `README.md` and `CONTRIBUTING.md` so I think it has some 'understanding' what it does. The idea is not that complicated. Actually I was surprised something like this did NOT exist yet. Perhaps it does, and I can't find it. But ChatGPT did advise to use the plugin `mdbook-collapsible` to get what I wanted. But it turned out that did NOT exist, it was a complete fabrication.

My use of ChatGPT does not mean my endorsing of this technology. I read Nick Bostrom's book [`Superintelligence: Paths, Dangers, Strategies` (2014)](https://en.wikipedia.org/wiki/Superintelligence:_Paths,_Dangers,_Strategies) and it scared me. Though I don't believe LLM's themselves will cause the singularity, they could be an important brick on the road to AGI. :scared

I believe LLM's will be a 10x factor, sooner than `Functional programming`. Read Mark Seemann's excellent post [`Yes, Silver Bullet`](https://blog.ploeh.dk/2019/07/01/yes-silver-bullet/) on this (2019).

## Contact

For any questions or issues, please contact the project maintainer at bartvanderwal at han dot nl.
