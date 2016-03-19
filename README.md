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
