mod programs;

#[cfg(test)]
mod tests {
  use solana_sdk;
  use solana_client::rpc_client::RpcClient;
  use solana_program::{ pubkey::Pubkey, system_instruction::transfer, system_program };
  use solana_sdk::{message::Message, signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction};
  use std::str::FromStr;
  use crate::tests::solana_sdk::blake3::hash;
  use crate::programs::turbin3_prereq::{TurbinePrereqProgram, CompleteArgs, UpdateArgs};

  const RPC_URL: &str = "https://api.devnet.solana.com";

  #[test]
  fn enroll() {
    let rpc_client = RpcClient::new(RPC_URL);
    let signer = read_keypair_file("Turbin3-wallet.json")
      .expect("Couldn't find wallet file");

    let prereq = TurbinePrereqProgram::derive_program_address(&[b"prereq",
      signer.pubkey().to_bytes().as_ref()]);

    let args = CompleteArgs { github: b"sabbCodes".to_vec() };

    let blockhash = rpc_client .get_latest_blockhash()
      .expect("Failed to get recent blockhash");

    let transaction = TurbinePrereqProgram::complete(
      &[&signer.pubkey(), &prereq, &system_program::id()],
      &args,
      Some(&signer.pubkey()),
      &[&signer],
      blockhash
    );

    let signature = rpc_client
      .send_and_confirm_transaction(&transaction)
      .expect("Failed to send transaction");

    println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
  }

  #[test]
  fn keygen() {
    let kp = Keypair::new();
    println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");

    println!("{:?}", kp.to_bytes());

  }

  #[test]
  fn airdrop() {
    let keypair = read_keypair_file("dev-wallet.json")
      .expect("Couldn't find wallet file");

    let client = RpcClient::new(RPC_URL);

    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {

      Ok(s) => {
        println!("Success! Check out your TX here:");

        println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
      },

      Err(e) => println!("Oops, something went wrong: {}", e.to_string())
    };
  }

  #[test]
  fn transfer_sol() {
    let keypair = read_keypair_file("dev-wallet.json")
      .expect("Couldn't find wallet file");

    let pubkey = keypair.pubkey();
    let message_bytes = b"I verify my solana Keypair!";
    let sig = keypair.sign_message(message_bytes);
    let sig_hashed = hash(sig.as_ref());

    match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
      true => println!("Signature verified"),
      false => println!("Verification failed"),
    }

    let to_pubkey = Pubkey::from_str("Sab5y7LG3VefLz4E6DSCkxdmjG4pve3hcAb8NUPKn42")
      .unwrap();

    let rpc_client = RpcClient::new(RPC_URL);
    let recent_blockhash = rpc_client .get_latest_blockhash()
      .expect("Failed to get recent blockhash");

    // let transaction = Transaction::new_signed_with_payer( &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);

    // Get balance of dev wallet
    let balance = rpc_client
      .get_balance(&keypair.pubkey())
      .expect("Failed to get balance");

    // Create a test transaction to calculate fees
    let message = Message::new_with_blockhash(&[transfer(&keypair.pubkey(), &to_pubkey, balance,)], Some(&keypair.pubkey()), &recent_blockhash);

    // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
    let fee = rpc_client.get_fee_for_message(&message)
      .expect("Failed to get fee calculator");

    let transaction = Transaction::new_signed_with_payer(&[transfer( &keypair.pubkey(), &to_pubkey, balance - fee)],
      Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);

    let signature = rpc_client
      .send_and_confirm_transaction(&transaction)
      .expect("Failed to send transaction");

    println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
  }
}