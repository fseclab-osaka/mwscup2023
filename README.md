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
などでコマンドを使用できる。

## 本コードの詳細
