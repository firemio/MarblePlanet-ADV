spl-tokenのCLIで作ったSPLトークンが既にあったとする。
それに対して、metadataを入れる。

https://developers.metaplex.com/token-metadata/getting-started/rust



```
mkdir solana-metadata
cd solana-metadata
npm init -y
```


```
npm install @solana/web3.js @metaplex/js ts-node typescript
```


```
npx tsc --init
```


```
ts-node set_metadata_for_spl_token.ts
```

