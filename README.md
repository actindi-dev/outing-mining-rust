# 開発環境

Rust の nightly を multirust でインストールする。

```
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
multirust update
multirust default nightly
```

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
