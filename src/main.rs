use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use mpl_token_metadata::{
    instructions::CreateMetadataAccountV3,
    types::{DataV2, Creator},
};
use solana_program::system_program;
use std::str::FromStr;

fn main() -> Result<()> {
    // Solana Mainnet Beta クライアントの初期化
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // ダミーの秘密鍵を使用してKeypairを生成
    let secret_key: [u8; 64] = [
        11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 101, 102, 103, 104, 105, 106,
        107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
        123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138,
        139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    ];
    let payer = Keypair::from_bytes(&secret_key)?;

    // 既存のトークンのミントアドレスを指定（実際のアドレスに置き換えてください）
    let mint_pubkey = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")?;

    // メタデータアカウントのアドレスを導出
    let metadata_seeds = &[
        b"metadata",
        mpl_token_metadata::ID.as_ref(),
        mint_pubkey.as_ref(),
    ];
    let (metadata_pubkey, _) = Pubkey::find_program_address(metadata_seeds, &mpl_token_metadata::ID);
    println!("Metadata Account Address: {}", metadata_pubkey);

    // メタデータの作成
    let data = DataV2 {
        name: "My Example Token".to_string(),
        symbol: "MET".to_string(),
        uri: "https://example.com/my_token_metadata.json".to_string(),
        seller_fee_basis_points: 0,
        creators: Some(vec![Creator {
            address: payer.pubkey(),
            verified: false,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    let create_metadata_ix = CreateMetadataAccountV3 {
        metadata: metadata_pubkey,
        mint: mint_pubkey,
        mint_authority: payer.pubkey(),
        payer: payer.pubkey(),
        update_authority: payer.pubkey(),
        system_program: system_program::id(),
        data,
        is_mutable: true,
        collection_details: None,
    }.instruction();

    // トランザクションの作成と送信
    let recent_blockhash = connection.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[create_metadata_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // トランザクションの送信
    let signature = connection.send_and_confirm_transaction(&transaction)?;
    println!("Transaction signature: {}", signature);

    Ok(())
}
