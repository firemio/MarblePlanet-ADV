use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use mpl_token_metadata::{
    instruction::create_metadata_accounts_v2,
    state::Data,
};
use std::str::FromStr;

fn main() {
    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let payer = Keypair::from_base58_string("YOUR_BASE58_PRIVATE_KEY");
    let mint_pubkey = Pubkey::from_str("YOUR_TOKEN_MINT_ADDRESS").unwrap();

    let metadata_seeds = &[
        b"metadata",
        &mpl_token_metadata::id().to_bytes(),
        &mint_pubkey.to_bytes(),
    ];
    let (metadata_pubkey, _) = Pubkey::find_program_address(metadata_seeds, &mpl_token_metadata::id());

    let metadata_data = Data {
        name: "Example Token".to_string(),
        symbol: "EXMPL".to_string(),
        uri: "https://example.com/path/to/metadata.json".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
    };

    let instructions = vec![
        solana_sdk::system_instruction::create_account(
            &payer.pubkey(),
            &metadata_pubkey,
            client.get_minimum_balance_for_rent_exemption(mpl_token_metadata::state::MAX_METADATA_LEN).unwrap(),
            mpl_token_metadata::state::MAX_METADATA_LEN as u64,
            &mpl_token_metadata::id(),
        ),
        create_metadata_accounts_v2(
            mpl_token_metadata::id(),
            metadata_pubkey,
            mint_pubkey,
            payer.pubkey(),
            payer.pubkey(),
            payer.pubkey(),
            metadata_data.name,
            metadata_data.symbol,
            metadata_data.uri,
            None,
            0,
            true,
            true,
        ),
    ];

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
    let recent_blockhash = client.get_recent_blockhash().unwrap().0;
    transaction.sign(&[&payer], recent_blockhash);

    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {}", signature);
}
