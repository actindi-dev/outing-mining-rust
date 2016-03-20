# 開発時

`features` に `watch` を指定して実行のビュー更新可にする。

```
RUST_BACKTRACE=1 cargo run --features 'watch serde_type'
```

## あっちの MongoDB につないでみたい場合

```
sudo service mongodb stop
ssh -C -L 27017:localhost:27017 log-iko-yo
```

# 本番環境

/etc/default/outing-mining-rust
で接続先の DB を指定する

```
DB_URL="mysql://user:password@host:port/db"
```

init スクリプトの仕込み

```
sudo ln -s /home/deployer/outing-mining-rust/etc/init.sh /etc/init.d/outing-mining-rust
sudo update-rc.d outing-mining-rust defaults
```
