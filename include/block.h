// dare — Block
#pragma once
#include <string>
#include <ctime>
#include <vector>
#include <openssl/sha.h>
#include <sstream>
#include <iomanip>
#include "transaction.h"
#include "merkle.h"

class Block {
    friend class Blockchain; // gives the Blockchain class access to Block's private fields
    public:
        //new Block constructor
        Block(int index, const std::vector<Transaction>& transactions, const std::string& previousHash)//member list
        : index_(index), timestamp_(std::time(nullptr)), transactions_(transactions), previousHash_(previousHash), hash_(calculateHash()) {} //initialized parameters when the cl

        //persistence (existing hash) Block constructor
        Block(int index, std::time_t timestamp, const std::vector<Transaction>& transactions, const std::string& previousHash, const std::string& hash)
        : index_(index), timestamp_(timestamp), transactions_(transactions), previousHash_(previousHash), hash_(hash) {} 

        //public getter methods
        int getIndex() const { return index_; }
        std::time_t getTimestamp() const { return timestamp_; }
        const std::string& getPreviousHash() const { return previousHash_; }
        const std::string& getHash() const { return hash_; }
        const std::vector<Transaction>&  getTransactions() const { return transactions_; }
    
        bool verifyHash() const {
            return calculateHash() == hash_;
        };
        
        bool verifyPreviousHash( const Block& previous) const {
            return getPreviousHash() == previous.getHash();
        };

    private:
        int index_;
        std::time_t timestamp_;
        std::vector<Transaction> transactions_;
        std::string previousHash_;
        std::string hash_;

        std::string calculateHash() const {
            std::vector<std::string> txIds;
            for (const Transaction& tx : transactions_) {
                txIds.push_back(tx.getTxId());
            }

            std::string txData = computeMerkleRoot(txIds);

            std::string input = std::to_string(index_) + std::to_string(timestamp_) + txData + previousHash_;

            unsigned char digest[SHA256_DIGEST_LENGTH];

            SHA256(reinterpret_cast<const unsigned char*>(input.c_str()), input.size(), digest);

            std::ostringstream oss;
            for (int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
                oss << std::hex << std:: setw(2) << std::setfill('0') << (int)digest[i];
            }

            return oss.str();
        }
};

