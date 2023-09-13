## 使い方
nushellにコマンドを追加するには以下のコマンドを実行
```
register (バイナリのファイル)
```
ただし，バイナリのファイル名は `nu_plugin_` の形である必要がある．

## 本コードのbuildについて
nushellコマンドのコードにはnu-plugin, nu-protocolが必要
```
 cargo add nu-plugin nu-protocol
```
`cargo build`によりバイナリを生成する。バイナリは `nu_plugin_test/target/debug` 内に生成される．