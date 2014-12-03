# Identicon

A port of GitHub's [identicon](https://en.wikipedia.org/wiki/Identicon) algorithm to Rust.

## Usage

```
$ echo -n 480938 | identicon hubot.png
$ identicon hubot.png < robots.txt
```

![hubot](https://cloud.githubusercontent.com/assets/122102/5274078/62b57c18-7a4d-11e4-90fa-46edd2ff7084.png)

## Development

```
$ cargo build
```

## License

Identicon is released under the MIT license. Check the LICENSE file for details.
