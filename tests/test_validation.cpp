#include <iostream>
#include <cassert>
#include <stdexcept>
#include "blockchain.h"

void test_valid_chain() {
    Blockchain bc;
    Wallet aliceWallet;
    Wallet bobWallet;
    bc.registerKey("Alice", aliceWallet.getPublicKey());
    bc.registerKey("Bob", bobWallet.getPublicKey());
    bc.fundAddress("Alice", 1000000, "USDC");
    bc.fundAddress("Bob", 1000000, "USDC");

    Transaction tx1("Alice", "Bob", 500000, "USDC");
    tx1.sign(aliceWallet);
    bc.addBlock({ tx1 });

    assert(bc.isValid() == true);
    std::cout << "PASS: valid chain" << std::endl;
}

//setting up wallets, adding a signed block, then tampering
void test_tampered_chain() {
    Blockchain bc;

    Wallet aliceWallet;
    bc.registerKey("Alice", aliceWallet.getPublicKey());
    bc.fundAddress("Alice", 100000, "USDC");

    Transaction tx1("Alice", "Bob", 50000, "USDC");
    tx1.sign(aliceWallet);
    bc.addBlock({ tx1 });

    bc.tamperBlock(1);
    assert(bc.isValid() == false);
    std::cout << "PASS: tampered chain detected" << std::endl;
}

//addBlock should throw when a sender doesn't have enough balance - using try/catch
void test_insufficient_funds() {
    Blockchain bc;

    Wallet aliceWallet;
    bc.registerKey("Alice", aliceWallet.getPublicKey());
    bc.fundAddress("Alice", 100000, "USDC");

    Transaction tx1("Alice", "Bob", 500000, "USDC");
    tx1.sign(aliceWallet);

    bool threw = false;

    try {
        bc.addBlock({ tx1 });
    } catch (const std::runtime_error&) {
        threw = true;
    }

    assert(threw == true);
    std::cout << "PASS: insufficient funds rejected" << std::endl;
}

//sign a transaction with the wrong wallet - addBlock should then throw
void test_invalid_signature() {
    Blockchain bc;

    Wallet aliceWallet;
    Wallet eveWallet; //attacker
    bc.registerKey("Alice", aliceWallet.getPublicKey());
    bc.fundAddress("Alice", 100000, "USDC");

    Transaction tx1("Alice", "Bob", 50000, "USDC");
    tx1.sign(eveWallet);

    bool threw = false;
    try {
        bc.addBlock({ tx1 });
    } catch (const std::runtime_error&) {
        threw = true;
    }

    assert(threw == true);
    std::cout << "PASS: invalid signature rejected" << std::endl;
}


int main () {
    test_valid_chain();
    test_tampered_chain();
    test_insufficient_funds();
    test_invalid_signature();
    std::cout << "\nAll tests passed." << std::endl;
    return 0;
}
