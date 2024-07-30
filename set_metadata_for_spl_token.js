import { Connection, PublicKey, Keypair, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import {
  createCreateMetadataAccountV3Instruction,
  createUpdateMetadataAccountV2Instruction,
  PROGRAM_ID,
} from '@metaplex-foundation/mpl-token-metadata';

const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed');

async function setOrUpdateMetadata(mintAddress, payer, name, symbol, uri) {
  const mintPublicKey = new PublicKey(mintAddress);
  
  const [metadataAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from('metadata'), PROGRAM_ID.toBuffer(), mintPublicKey.toBuffer()],
    PROGRAM_ID
  );

  // メタデータアカウントの存在確認
  const metadataAccountInfo = await connection.getAccountInfo(metadataAccount);

  let instruction;
  if (metadataAccountInfo === null) {
    // メタデータアカウントが存在しない場合、新規作成
    instruction = createCreateMetadataAccountV3Instruction(
      {
        metadata: metadataAccount,
        mint: mintPublicKey,
        mintAuthority: payer.publicKey,
        payer: payer.publicKey,
        updateAuthority: payer.publicKey,
      },
      {
        createMetadataAccountArgsV3: {
          data: {
            name,
            symbol,
            uri,
            sellerFeeBasisPoints: 0,
            creators: null,
            collection: null,
            uses: null,
          },
          isMutable: true,
          collectionDetails: null,
        },
      }
    );
  } else {
    // メタデータアカウントが存在する場合、更新
    instruction = createUpdateMetadataAccountV2Instruction(
      {
        metadata: metadataAccount,
        updateAuthority: payer.publicKey,
      },
      {
        updateMetadataAccountArgsV2: {
          data: {
            name,
            symbol,
            uri,
            sellerFeeBasisPoints: 0,
            creators: null,
            collection: null,
            uses: null,
          },
          updateAuthority: payer.publicKey,
          primarySaleHappened: null,
          isMutable: true,
        },
      }
    );
  }

  const transaction = new Transaction().add(instruction);
  try {
    const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);
    console.log('Metadata set or updated successfully:', signature);
  } catch (error) {
    console.error('Error setting or updating metadata:', error);
  }
}

async function main() {
  try {
    const mintAddress = 'YOUR_MINT_ADDRESS_HERE';
    const payer = Keypair.fromSecretKey(new Uint8Array(JSON.parse('YOUR_SECRET_KEY_HERE')));
    const name = "Your Token Name";
    const symbol
