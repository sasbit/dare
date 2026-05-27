// dare — stablecoin payments chain prototype

#include <iostream>
#include "blockchain.h"
#include "transaction.h"
#include "wallet.h"
#include "persistence.h"

int main() {
    std::cout << "chain starting..."  << std::endl;

    Blockchain bc;

    std::vector<Transaction> block1Txs = {
        Transaction("Sascha", "Franzi", 1000000, "USDC"),
        Transaction("Bob", "Alice", 500000, "EURC"),
    };

    //creating wallets and registering publicKeys
    Wallet saschaWallet;
    Wallet bobWallet;

    bc.registerKey("Sascha", saschaWallet.getPublicKey());
    bc.registerKey("Bob", bobWallet.getPublicKey());  
    
    bc.fundAddress("Sascha", 5000000, "USDC");
    bc.fundAddress("Bob", 2000000, "EURC");
    
    //signing before adding the block
    block1Txs[0].sign(saschaWallet);
    block1Txs[1].sign(bobWallet);

    bc.addBlock(block1Txs);

    saveChain(bc);
    std::cout << "Chain saved." << std::endl;

    //load into a fresh chain and verify
    Blockchain bc2 = loadChain();
    std::cout << "Loaded Chain: " << std::endl;
    for (const Block& b : bc2.getChain()) {
        std::cout << "Block " << b.getIndex() << " | Hash: " << b.getHash() << std::endl;
        for (const Transaction& tx: b.getTransactions()) {
            std::cout << "  " << tx.getSender() << " -> " << tx.getReceiver() << " " << tx.getAmount() << " " << tx.getCurrency() << std::endl;

        }
    }

    std::cout << "Loaded chain valid: " << bc2.isValid() << std::endl;

    //looping over the chain - printing each Block's index and for each block looping over its transactions and print sender, receiver, amount, currency
    for (const Block& b : bc.getChain()) {
        std::cout << "Block " << b.getIndex() << " | Hash: " << b.getHash() << std::endl;

        for (const Transaction& tx : b.getTransactions()) {
            std::cout << "Transaction " << tx.getTxId() << " | " << tx.getSender() << " | " 
            << tx.getReceiver() << " | " << tx.getAmount() << " | " << tx.getCurrency() << std::endl;
        }
    }

    std::cout << "Sascha USDC balance: " << bc.getBalance("Sascha", "USDC") << std::endl;
    std::cout << "Bob EURC balance: " << bc.getBalance("Bob", "EURC") << std::endl;
    std::cout << "Chain Valid: " << bc.isValid() << std::endl;
    return 0;
}

