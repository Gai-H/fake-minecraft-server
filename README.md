# fake-minecraft-server
Rust の練習として作った見せかけの Minecraft サーバ

[Ping シーケンス](https://wiki.vg/Protocol_FAQ#What_does_the_normal_status_ping_sequence_look_like.3F) の全てと、
[Login シーケンス](https://wiki.vg/Protocol_FAQ#What.27s_the_normal_login_sequence_for_a_client.3F) の Encryption Response を受け取るまでを実装している
(Encryption Response パケットを受けた瞬間に [Disconnect パケット](https://wiki.vg/Protocol#Disconnect_.28login.29) を送ってキックする)

<div><video controls src="https://github.com/Gai-H/fake-minecraft-server/assets/23699120/cdcf92da-02c1-434a-8759-4ca24bfccde3"></video></div>

## Requirements
OpenSSL - [プロトコル暗号化](https://wiki.vg/Protocol_Encryption) のため

## Configuration
サーバからの応答を変更できるようにしている

プロジェクトルートに `Config.toml` を作成して、以下の項目を記述することで変更できる
| 項目名 | 型 | 説明 |
| --- | --- | --- |
|`port`|`u16`|ポート番号|
|`version-name`|`String`|バージョン名|
|`version-protocol`|`u16`|[プロトコルのバージョン](https://wiki.vg/Protocol_version_numbers)|
|`description`|`String`|サーバの説明|
|`players-max`|`u16`|プレイヤー数の上限|
|`players-online`|`u16`|参加中のプレイヤー数|
|`disconnect-reason`|`String`|キック時に表示される文章|
|`command`|`[String]`|ステータスもしくはログインのリクエストが成功したときに実行するコマンド <br> 以下の変数は置換される <br> `%peer-address%` - 例: `127.0.0.1:12345` <br> `%username%` - 例: `Notch` <br> `%uuid%` - 例: `069a79f444e94726a5befca90e38aaf5` <br> `%state%` - `STATUS` or `LOGIN` <br> `%is_authenticated%` - `true` or `false`|


## References
- [Minecraft Modern (wiki.vg)](https://wiki.vg/Main_Page)
- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/)
