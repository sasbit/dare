use dare::blockchain::Blockchain;
use dare::transaction::Transaction;
use dare::wallet::Wallet;

fn setup() -> (Blockchain, Wallet, Wallet, String, String) {
    let alice_wallet = Wallet::new();
    let bob_wallet   = Wallet::new();
    let alice = String::from("alice");
    let bob   = String::from("bob");

    let mut bc = Blockchain::new();
    bc.register_key(alice.clone(), alice_wallet.public_key());
    bc.register_key(bob.clone(),   bob_wallet.public_key());
    bc.fund_address(alice.clone(), String::from("USDC"), 1_000_000);

    (bc, alice_wallet, bob_wallet, alice, bob)
}

#[test]
fn test_valid_chain() {
    let (mut bc, alice_wallet, _bob_wallet, alice, bob) = setup();

    let mut tx = Transaction::new(alice, bob, 250_000, String::from("USDC"));
    tx.sign(&alice_wallet);
    bc.add_block(vec![tx]).unwrap();

    assert!(bc.is_valid());
}

#[test]
fn test_tampered_chain() {
    let (mut bc, alice_wallet, _bob_wallet, alice, bob) = setup();

    let mut tx = Transaction::new(alice, bob, 250_000, String::from("USDC"));
    tx.sign(&alice_wallet);
    bc.add_block(vec![tx]).unwrap();

    bc.tamper_block(1);
    assert!(!bc.is_valid());
}

#[test]
fn test_insufficient_funds() {
    let (mut bc, alice_wallet, _bob_wallet, alice, bob) = setup();

    let mut tx = Transaction::new(alice, bob, 2_000_000, String::from("USDC"));
    tx.sign(&alice_wallet);

    assert!(bc.add_block(vec![tx]).is_err());
}

#[test]
fn test_invalid_signature() {
    let (mut bc, _alice_wallet, bob_wallet, alice, bob) = setup();

    let mut tx = Transaction::new(alice, bob, 250_000, String::from("USDC"));
    tx.sign(&bob_wallet); // signed with wrong wallet

    assert!(bc.add_block(vec![tx]).is_err());
}
