This is a tool for updating the [Karabiner-Elements](https://karabiner-elements.pqrs.org/) configuration file[^1].

You can write `Karabiner-Elements` configuration in Rust.

### Usage

Run the following command to update the `Karabiner-Elements` configuration file based on the current code in this repository:

```shell
$ cargo run
```

### Others

My import link:

```
karabiner://karabiner/assets/complex_modifications/import?url=https://raw.githubusercontent.com/hioki/karaconf/main/custom.json
```

[^1]: For example, `~/.config/karabiner/assets/complex_modifications/custom.json` or `~/.config/karabiner/karabiner.json. For details`, see: https://karabiner-elements.pqrs.org/docs/json/location/
