## 使い方
nushellにコマンドを追加するには以下のコマンドを実行
```
register (バイナリのファイル)
```
ただし，バイナリのファイル名は `nu_plugin_` の形である必要がある．

## 本コードのbuildについて
nushellコマンドのコードにはnu-plugin, nu-protocolが必要(Cargo.tomlを参照)です．
これを
```
nu-plugin = "0.84.0"
```
の形でパッケージを用いるとbuildがうまくできない．(これはコードが悪いかもしれない)
そこで，nushellのソースコード中で，cratesディレクトリ内にnu_plugin_nopenディレクトリをコピーし，コピーしたnu_plugin_nopenディレクトリ内で`cargo build`によりbuildすることで，ビルドできます．
この際，バイナリは `nushell/target/debug` 内に生成される．
また， `nushell/` 内のCargo.tomlの[workspace]に追記する必要がある．これも一緒にあげているので，参考にしてください．

## 注意
該当バイナリについては，自分のローカル環境での動作は確認しています．
バイナリはarm, ~~x86を用意してあります．~~(arm, x86ディレクトリ内)x86バイナリはクロスコンパイルの弊害か100MB超えてgitに上げれなくなったので，drive(https://drive.google.com/drive/folders/1MNDo4vSJpxHdujInHk7emdv3K4-gA5Qm?usp=sharing)にあげてあります．
しかし，arm環境なのでarmのホスト環境, x86の仮想環境での確認なので，x86マシンでの動作確認はできてません．(多分windowsだと動かないので自分でビルドするか，linuxにしてみてください)
無理そうなら，ビルドから試してもらえるとありがたいです．

## コマンドについて
現状はnopenコマンドが追加されるようになります．
コマンドの追加ができることをまず検証したかったので，本コマンドはファイルのパスを受け取り，それが存在するかどうかを判定するのみのコマンドです．
ただし，動作として絶対パス，相対パスどちらも動作しますが"\~"を用いたパス(\~/Documents/... のようなもの)に対してうまく動作がしません．
これについては，ログファイルの受け取りにも関わると思うので，対策が必要かもしれません．