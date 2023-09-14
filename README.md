## 使い方
`git clone`をして`nu_plugin_test`ディレクトリで`cargo build`を行う。buildしたバイナリは`nu_plugin_test/target/debug/`内に生成される。  
nushellにコマンドを追加するには以下のコマンドを実行
```
register (バイナリのファイル)
```
ただし，バイナリのファイル名は `nu_plugin_` の形である必要がある。登録したコマンドは`~/.config/nushell/plugin.nu`に設定が保存される。
```
test-dayo 1 a 12 --flag afd aa --named af a
```
などでコマンドを使用できる。`-h`でヘルプが見れる。

## 本コードの詳細
### lib.rs
ライブラリクレートであり、`mod`でmoduleを読み込み、`pub use`で他のファイルで扱えるようにする。
main.rsでは`(package name)::`、他のファイルでは`crate::`で扱える。

### main.rs
serve_pluginを実行するだけ。TestはPluginトレイトが実装されている。encoderには`MsgPackSerializer`か`JsonSerializer`を選ぶ。違いはよく分からないが`MsagPackSerializer`の方が軽いらしい。

### test.rs
コマンドの処理が書かれている。`call`オブジェクトによりコマンドライン引数を受け取る

### nu.rs
signature関数により、`-h`フラグで表示されるヘルプ内容をtest.rsの`call`オブジェクトに対応するように設定する。`~/.config/nushell/plugin.nu`に保存される。