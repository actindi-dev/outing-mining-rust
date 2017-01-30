# 開発環境

## コンパイラ

rustup で rust をインストールする。
https://www.rustup.rs/

```
curl https://sh.rustup.rs -sSf | sh
```

nightly-2017-01-24 を使っているので次のようにバージョンを固定する。

```
rustup override set nightly-2017-01-24
```

## 開発用 OAuth の設定

https://console.developers.google.com/apis/dashboard で新しいプロジェクトを作成する。

* 認証情報
  * アプリケーションの種類
    * OAuth クライアント ID
  * 名前
    * 任意
  * 承認済みの JavaScript 生成元
    * http://127.0.0.1:1958
  * 承認済みのリダイレクト URI
    * http://127.0.0.1:1958/oauth2callback

google-oauth.mk に クライアントID、クライアント シークレット、承認済みのリダイレクト URI を書く。

```
OAUTH_CLIENT_ID="999999999999-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.apps.googleusercontent.com"
OAUTH_CLIENT_SECRET="xxxxxxxxxxxxxxxxxxxxxxxx"
OAUTH_REDIRECT_URI="http://localhost:1958/oauth2callback"
```

## ビルド

`src` ディレクトリで `make` すれば動く。


# 本番環境

/etc/default/outing-mining-rust
で接続先の DB、OAuth 情報を指定する

```
export DB_URL="mysql://user:password@host:port/db"
export OAUTH_CLIENT_ID="xxxxxxxxxxxxxxxxxxxx"
export OAUTH_CLIENT_SECRET="xxxxxxxxxxxxxxxxxxx"
export OAUTH_REDIRECT_URI="https://www.example.com/oauth2callback"
```

init スクリプトの仕込み

```
sudo ln -s /home/deployer/outing-mining-rust/etc/init.sh /etc/init.d/outing-mining-rust
sudo update-rc.d outing-mining-rust defaults
```

# デプロイ

```
make deploy
```

再起動はしないので `ssh` して `sudo service outing-mining-rust restart` する。


# その他

## あっちの MongoDB につないでみたい場合

```
sudo service mongodb stop
ssh -C -L 27017:localhost:27017 log-iko-yo
```
