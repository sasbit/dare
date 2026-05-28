use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use crate::blockchain::Blockchain;
use crate::message::Message;

pub struct Node{ 
    pub address: String,
    pub blockchain: Arc<Mutex<Blockchain>>,
    peers: Arc<Mutex<Vec<String>>>,
}

impl Node {
    pub fn new(address: String, blockchain: Arc<Mutex<Blockchain>>) -> Self {
        Node {
            address,
            blockchain,
            peers: Arc::new(Mutex::new(vec![])),
        }
    }

    //async listener loop, &self argument is a read-only borrow of Self - doesn't modify the Node itself, just spawns tasks that share its Arc handles
    pub async fn listen(&self) {
        let listener = TcpListener::bind(&self.address).await.unwrap();
        println!("listening on {}", self.address);

        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            println!("new connection from {}", addr);

            let blockchain = Arc::clone(&self.blockchain);
            let peers = Arc::clone(&self.peers);

            tokio::spawn(async move {
                handle_connection(stream, blockchain, peers).await;
            });
        }
    }

    pub async fn register_peer(&self, peer_addr: &str) {
        self.peers.lock().await.push(peer_addr.to_string());
        println!("registered peer {}", peer_addr);
    }

    pub async fn broadcast(&self, message: &Message) { 
        let peers = self.peers.lock().await.clone();
        let line = serde_json::to_string(message).unwrap() + "\n";

        for peer_addr in &peers {
            if let Ok(mut stream) = TcpStream::connect(peer_addr).await {
                let _ = stream.write_all(line.as_bytes()).await;
            }
        }
    }
}

async fn handle_connection(stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>, _peers: Arc<Mutex<Vec<String>>>) {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.unwrap_or(None) {
        let msg: Message = match serde_json::from_str(&line) {
            Ok(m) => m,
            Err(_) => continue,
        };
        
        match msg {
            Message::NewTransaction(tx) => {
                println!("received transaction: {}", tx.tx_id());
            }
            Message::NewBlock(block) => {
                println!("received block: {}", block.index());
                blockchain.lock().await.load_block(
                    block.index(),
                    block.timestamp(),
                    block.transactions().to_vec(),
                    block.previous_hash().to_string(),
                    block.hash().to_string(),
                );
            }
            Message::RequestChain => {
                println!("chain requested");
            }
            Message::ResponseChain(_) => {
                println!("chain received");
            }
            Message::Hello { from } => {
                println!("hello from {}", from);
            }
        }
    }
}