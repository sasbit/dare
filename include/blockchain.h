// dare — Blockchain
#pragma once
#include <vector>
#include <map>
#include <stdexcept>
#include "block.h"
#include "transaction.h"
#include "wallet.h"

class Blockchain {
    private:
        std::vector<Block> chain_;
        std::map<std::string, std::map<std::string, uint64_t>> balances_;
        std::map<std::string, std::vector<unsigned char>> publicKeys_;

        void rebuildBalances() {
            balances_.clear();
            for (const Block& b : chain_) {
                for (const Transaction& tx : b.getTransactions()) {
                    balances_[tx.sender_][tx.currency_] -= tx.amount_;
                    balances_[tx.receiver_][tx.currency_] += tx.amount_;
                }
            }

        }
    
    public:
        // tag-dispatch marker: lets persistence.h construct a Blockchain
        // WITHOUT auto-creating a genesis block (the genesis comes from the file).
        struct LoadTag {};

        //constructor
        Blockchain() {
            chain_.push_back(Block(0, std::vector<Transaction>{}, "0"));
        }

        //load-mode constructor: leaves `chain_` empty so loadBlock can populate it.
        explicit Blockchain(LoadTag) {}

        void registerKey(const std::string& address, const std::vector<unsigned char>& publicKey) {
            publicKeys_[address] = publicKey;
        }
        
        //fundAdress() method for initializing balances
        void fundAddress(const std::string& address, uint64_t amount, const std::string& currency) {
            balances_[address][currency] += amount;
        }

        uint64_t getBalance(const std::string& address, const std::string& currency) const {
            auto it = balances_.find(address);
            if (it == balances_.end()) {
                return 0;
            }
            auto it2 = it->second.find(currency);
            if (it2 == it->second.end()) {
                return 0;
            }

            return it2->second;
        }

        const std::vector<Block> & getChain() const { return chain_; }
    
        void addBlock(const std::vector<Transaction>& transactions) {
            //wallet signature verification
            for (const Transaction& tx : transactions) {
                auto it = publicKeys_.find(tx.sender_);
                if (it == publicKeys_.end()){
                    throw std::runtime_error(tx.sender_ + " has no registered public key");
                }

                if (!tx.verify(it->second)) {
                    throw std::runtime_error("invalid signature from " + tx.sender_);
                }
            }
            
            for (const Transaction& tx : transactions) {
                if (getBalance(tx.sender_, tx.currency_) < tx.amount_) {
                    throw std::runtime_error(tx.sender_ + " has insufficient funds");
                }
            }
            
            for (const Transaction& tx : transactions) {
                balances_[tx.sender_][tx.currency_] -= tx.amount_;
                balances_[tx.receiver_][tx.currency_] += tx.amount_;
            }

            int newIndex = static_cast<int>(chain_.size());
            const std::string & lastHash = chain_.back().getHash();
            chain_.push_back(Block(newIndex, transactions, lastHash));
        }

        void loadBlock(int index, std::time_t timestamp, const std::vector<Transaction>& transactions, const std::string& previousHash, const std::string& hash) {
            chain_.push_back(Block(index, timestamp, transactions, previousHash, hash));

            rebuildBalances();
        }

        bool isValid () const {
            for (size_t i = 1; i < chain_.size(); i++) {
                //check1
                if(!chain_[i].verifyHash()) {
                    return false;
                }
                
                if (!chain_[i].verifyPreviousHash(chain_[i-1])) {
                    return false;
                }
            }
            return true;
        }

        //test function to corrupt an existing Block with new data
        void tamperBlock(int index) {
            chain_[index].transactions_[0].txId_ = "corrupted";
        }
};