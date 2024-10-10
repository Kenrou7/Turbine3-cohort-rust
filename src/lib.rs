#[cfg(test)]
mod programs;

mod tests {
    use solana_sdk::{message::Message, signature::{read_keypair_file, Keypair, Signer}, system_program, transaction:: Transaction};
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::system_instruction::transfer;
    use std::vec;
    use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs};

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let base58 = "gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7VvksUGhPt4SZ8JHVSkt";
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let wallet = [34, 46, 55, 124, 141, 190, 24, 204, 134, 91, 70, 184, 161, 181, 44, 122, 15, 172, 63, 62, 153, 150, 99, 255, 202, 89, 105, 77, 41, 89, 253, 130, 27, 195, 134, 14, 66, 75, 242, 7, 132, 234, 160, 203, 109, 195, 116, 251, 144, 44, 28, 56, 231, 114, 50, 131, 185, 168, 138, 61, 35, 98, 78, 53];
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);   
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client: RpcClient = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => {
                println!("Oops, something went wrong, {}", e.to_string());
            }
        }
    }
    #[test]
    fn transfer_sol() {
        let dev_wallet_keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let wallet_keypair = read_keypair_file("wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = wallet_keypair.pubkey();
        let rpc_client: RpcClient = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
        let transaction = Transaction::new_signed_with_payer( 
            &[transfer(
                &dev_wallet_keypair.pubkey(), 
                &to_pubkey, 
                100_000_000
            )
            ], 
            Some(&dev_wallet_keypair.pubkey()), 
            &vec![&dev_wallet_keypair], 
            recent_blockhash
        );
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }

    #[test]
    fn empty_account() {
        let dev_wallet_keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let wallet_keypair = read_keypair_file("wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = wallet_keypair.pubkey();

        let rpc_client: RpcClient = RpcClient::new(RPC_URL);
        let balance = rpc_client.get_balance(&dev_wallet_keypair.pubkey()).expect("Failed to get balance");
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(
                &dev_wallet_keypair.pubkey(),
                &to_pubkey,
                balance
            )], 
            Some(&dev_wallet_keypair.pubkey()), 
            &recent_blockhash
        );

        let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &dev_wallet_keypair.pubkey(),
                &to_pubkey,
                balance - fee
            )], 
            Some(&dev_wallet_keypair.pubkey()), 
            &vec![&dev_wallet_keypair],
            recent_blockhash, 
        );

        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }

    #[test]
    fn submit() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("wallet.json").expect("Couldn't find wallet file");
        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
        let args = CompleteArgs {
            github: b"Kenrou7".to_vec()
        };
        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let transaction = Turbin3PrereqProgram::complete(
            &[
                &signer.pubkey(),
                &prereq,
                &system_program::id()
            ],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );

        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }


}
