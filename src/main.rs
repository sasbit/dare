use dare::blockchain::Blockchain;
use dare::transaction::Transaction;
use dare::wallet::Wallet;

fn main() {
    //1. creating wallets
    let alice_wallet = Wallet::new();
    let bob_wallet = Wallet::new();

    let alice = String::from("alice");
    let bob = String::from("bob");

    //2. spinning up chain, registering keys, fund alice
    let mut bc = Blockchain::new();
    bc.register_key(alice.clone(), alice_wallet.public_key());
    bc.register_key(bob.clone(),   bob_wallet.public_key());
    bc.fund_address(alice.clone(), String::from("USDC"), 1_000_000);

    println!("alice: {}", bc.get_balance(&alice, "USDC"));
    println!("bob: {}", bc.get_balance(&bob, "USDC"));

    //3. create, sign and submit a transaction
    let mut tx = Transaction::new(alice.clone(), bob.clone(), 250_000, String::from("USDC"));
    tx.sign(&alice_wallet);

    match bc.add_block(vec![tx]) {
        Ok(()) => println!("block added"),
        Err(e) => println!("error: {}", e),
    }

    println!("alice: {}", bc.get_balance(&alice, "USDC"));
    println!("bob:   {}", bc.get_balance(&bob,   "USDC"));

    // 4. validate chain
    println!("chain valid: {}", bc.is_valid());



}
