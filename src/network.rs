use std::mem;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use crate::blockchain::Blockchain;
use crate::message::Message;
use crate::transaction::Transaction;

pub struct Node{ 
    pub address: String,
    pub blockchain: Arc<Mutex<Blockchain>>,
    peers: Arc<Mutex<Vec<String>>>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Node {
    pub fn new(address: String, blockchain: Arc<Mutex<Blockchain>>) -> Self {
        Node {
            address,
            blockchain,
            peers: Arc::new(Mutex::new(vec![])),
            mempool: Arc::new(Mutex::new(vec![])),
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
            let mempool = Arc::clone(&self.mempool);

            tokio::spawn(async move {
                handle_connection(stream, blockchain, peers, mempool).await;
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

async fn handle_connection(stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>, peers: Arc<Mutex<Vec<String>>>, mempool: Arc<Mutex<Vec<Transaction>>>) {
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
                mempool.lock().await.push(tx);
                try_produce_block(
                    Arc::clone(&blockchain),
                    Arc::clone(&peers),
                    Arc::clone(&mempool),
                ).await;
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

async fn try_produce_block(
    blockchain: Arc<Mutex<Blockchain>>,
    peers: Arc<Mutex<Vec<String>>>,
    mempool: Arc<Mutex<Vec<Transaction>>>,
) {
    const BLOCK_SIZE: usize = 1;

    // Drain the mempool if it has reached BLOCK_SIZE; otherwise wait for more txs.
    let txs: Vec<Transaction> = {
        let mut pool = mempool.lock().await;
        if pool.len() < BLOCK_SIZE {
            return;
        }
        mem::take(&mut *pool)
    };

    // Add the block to the chain and grab a clone for broadcasting.
    let block = {
        let mut bc = blockchain.lock().await;
        if let Err(e) = bc.add_block(txs) {
            println!("block production failed: {}", e);
            return;
        }
        bc.chain().last().unwrap().clone()
    };

    println!("produced block {}", block.index());

    // Gossip the new block to all known peers.
    let peer_list = peers.lock().await.clone();
    let line = serde_json::to_string(&Message::NewBlock(block)).unwrap() + "\n";
    for peer_addr in &peer_list {
        if let Ok(mut stream) = TcpStream::connect(peer_addr).await {
            let _ = stream.write_all(line.as_bytes()).await;
        }
    }
}
