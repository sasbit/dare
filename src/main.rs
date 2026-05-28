use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;
use dare::blockchain::Blockchain;
use dare::transaction::Transaction;
use dare::wallet::Wallet;
use dare::network::Node;
use dare::message::Message;

//tokio for async execution
#[tokio::main]
async fn main() {
    //registering Blockchain in Arc<Mutex<>> - Arc allows multiple owners, Mutex ensures only one task writes at a time
    let chain1 = Arc::new(Mutex::new(Blockchain::new()));
    let chain2 = Arc::new(Mutex::new(Blockchain::new()));

    //setting up wallets and funding
    let alice_wallet = Wallet::new();
    let alice = String::from("alice");
    let bob = String::from("bob");

    {
        let mut bc = chain1.lock().await;
        bc.register_key(alice.clone(), alice_wallet.public_key());
        bc.register_key(bob.clone(), Wallet::new().public_key());
        bc.fund_address(alice.clone(), String::from("USDC"), 1_000_000);
    }

    {
        let mut bc = chain2.lock().await;
        bc.register_key(alice.clone(), alice_wallet.public_key());
        bc.fund_address(alice.clone(), String::from("USDC"), 1_000_000);
    }    

    //spinning up nodes
    let node1 = Arc::new(Node::new(String::from("127.0.0.1:8001"), Arc::clone(&chain1)));
    let node2 = Arc::new(Node::new(String::from("127.0.0.1:8002"), Arc::clone(&chain2)));

    //spawning the listener tasks
    let node1_ref = Arc::clone(&node1);
    let node2_ref = Arc::clone(&node2);

    tokio::spawn(async move { node1_ref.listen().await });
    tokio::spawn( async move { node2_ref.listen().await });

    //sleep gives listeners 100Ms to bind their ports before reconnect attempt 
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    //registering peers and broadcasting to peers (register_peer takes the peer address)
    node1.register_peer("127.0.0.1:8002").await;
    node2.register_peer("127.0.0.1:8001").await;

    let mut tx = Transaction::new(alice.clone(), bob.clone(),1_000, String::from("USDC"));
    tx.sign(&alice_wallet);

    println!("broadcasting transaction ... ");
    node1.broadcast(&Message::NewTransaction(tx)).await;
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("done");

    // node3 joins late and syncs from node1
    let chain3 = Arc::new(Mutex::new(Blockchain::new()));
    {
        let mut bc = chain3.lock().await;
        bc.register_key(alice.clone(), alice_wallet.public_key());
        bc.fund_address(alice.clone(), String::from("USDC"), 1_000_000);
    }

    let node3 = Arc::new(Node::new(String::from("127.0.0.1:8003"), Arc::clone(&chain3)));
    let node3_ref = Arc::clone(&node3);
    tokio::spawn(async move { node3_ref.listen().await });
    tokio::time::sleep(Duration::from_millis(100)).await;

    println!("node3 syncing from node1...");
    node3.sync_chain_from("127.0.0.1:8001").await;

    println!(
        "node3 chain length: {}, valid: {}",
        chain3.lock().await.chain().len(),
        chain3.lock().await.is_valid()
    );
    
}