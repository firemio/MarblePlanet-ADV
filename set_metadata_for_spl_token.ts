import {
  Connection,
  PublicKey,
  Keypair,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import { Metadata, MetadataDataData, CreateMetadataV2 } from '@metaplex/js';
import { TOKEN_METADATA_PROGRAM_ID } from '@metaplex/js/lib/utils';

const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed');

// メタデータを設定する関数
async function setMetadata(mintAddress: string, metadataUri: string, payer: Keypair) {
  const mintPublicKey = new PublicKey(mintAddress);

  // メタデータアカウントの生成
  const metadataAccount = await Metadata.getPDA(mintPublicKey);

  // メタデータの設定トランザクション
  const transaction = new Transaction().add(
    new CreateMetadataV2(
      { feePayer: payer.publicKey },
      {
        metadata: metadataAccount,
        metadataData: new MetadataDataData({
          name: "Your Token Name",
          symbol: "TOKEN",
          uri: metadataUri,
          sellerFeeBasisPoints: 500,
          creators: null,
        }),
        updateAuthority: payer.publicKey,
        mint: mintPublicKey,
      }
    )
  );

  // トランザクションの送信
  const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);
  console.log('Metadata set successfully:', signature);
}

// メイン関数
(async () => {
  const mintAddress = 'YOUR_MINT_ADDRESS_HERE'; // 対象のトークンのミントアドレスを入力
  const payer = Keypair.fromSecretKey(new Uint8Array(JSON.parse('YOUR_SECRET_KEY_HERE'))); // 秘密鍵を入力

  const metadataUri = 'https://gateway.pinata.cloud/ipfs/YOUR_METADATA_JSON_HASH'; // メタデータJSONのURLを指定

  await setMetadata(mintAddress, metadataUri, payer);
})();
