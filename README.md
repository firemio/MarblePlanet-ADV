spl-tokenのCLIで作ったSPLトークンが既にあったとする。
それに対して、metadataを入れる。

https://developers.metaplex.com/token-metadata/getting-started/rust



```
mkdir solana-metadata
cd solana-metadata
npm init -y
```


```
npm install \
  @metaplex-foundation/umi \
  @metaplex-foundation/umi-bundle-defaults \
  @solana/web3.js \
  @metaplex-foundation/mpl-token-metadata
```

```
node set_metadata_for_spl_token.js
```

