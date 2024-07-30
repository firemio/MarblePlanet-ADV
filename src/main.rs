use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use mpl_token_metadata::instruction as metadata_instruction;

fn main() -> Result<()> {
    // Solana Mainnet Beta クライアントの初期化
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // トークン作成者のキーペア（実際の使用時は安全に管理してください）
    let payer = Keypair::from_base58_string("あなたの秘密鍵をここに入力"); // 秘密鍵を適切に設定

    // 既存のトークンのミントアドレスを指定（実際のアドレスに置き換えてください）
    let mint_pubkey = Pubkey::from_str("あなたのトークンミントアドレスをここに入力")?; // 例: "TokenMintAddress"

    // メタデータアカウントのアドレスを導出
    let metadata_seeds = &[
        "metadata".as_bytes(),
        mpl_token_metadata::ID.as_ref(),
        mint_pubkey.as_ref(),
    ];
    let (metadata_pubkey, _) = Pubkey::find_program_address(metadata_seeds, &mpl_token_metadata::ID);
    println!("Metadata Account Address: {}", metadata_pubkey);

    // メタデータの作成
    let create_metadata_ix = metadata_instruction::create_metadata_accounts_v3(
        mpl_token_metadata::ID,
        metadata_pubkey,
        mint_pubkey,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        "My Example Token".to_string(),
        "MET".to_string(),
        "https://example.com/my_token_metadata.json".to_string(), // 有効なメタデータURIに置き換え
        None,
        0,
        true,
        true,
        None,
        None,
        None,
    );

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
