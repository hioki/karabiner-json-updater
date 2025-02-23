[Karabiner-Elements](https://karabiner-elements.pqrs.org/) の設定ファイル[^1]を更新するツールです。

`Karabiner-Elements` の設定を Rust で書くことができます。

### 使い方

以下を実行すると、本レポジトリの現在のコードの内容に従って `Karabiner-Elements` の設定ファイルを更新します。

```shell
$ cargo run
```

### その他

私の設定のインポート用リンク:

```
karabiner://karabiner/assets/complex_modifications/import?url=https://raw.githubusercontent.com/hioki/karabiner-json-updater/main/custom.json
```

[^1]: `~/.config/karabiner/assets/complex_modifications/custom.json` や `~/.config/karabiner/karabiner.json` など。詳細: https://karabiner-elements.pqrs.org/docs/json/location/
