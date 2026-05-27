#pragma once
#include <string>
#include <cstdint>
#include <openssl/sha.h>
#include <sstream>
#include <iomanip>
#include <vector>
#include "wallet.h"

class Transaction {
    friend class Blockchain;
    public:
        Transaction(
            const std::string & sender,
            const std::string & receiver,
            uint64_t amount, 
            const std::string & currency) : sender_(sender), receiver_(receiver), amount_(amount), currency_(currency), txId_(computeId()) {}
        
        const std::string& getTxId() const { return txId_; }
        const std::string& getSender() const { return sender_; }
        const std::string& getReceiver() const { return receiver_; }
        uint64_t getAmount() const { return amount_; }
        const std::string& getCurrency() const { return currency_; }
            
        void sign(const Wallet& wallet) {
            std::string message = sender_ + receiver_ + std::to_string(amount_) + currency_;
            signature_ = wallet.sign(message);
        }

        bool verify(const std::vector<unsigned char>& publicKey) const {
            std::string message = sender_ + receiver_ + std::to_string(amount_) + currency_;

            EVP_PKEY* pubKey = EVP_PKEY_new_raw_public_key(EVP_PKEY_ED25519, nullptr, publicKey.data(), publicKey.size());
            if (!pubKey) return false;
            EVP_MD_CTX* mdctx = EVP_MD_CTX_new();
            if (!mdctx) { EVP_PKEY_free(pubKey); return false; }
            EVP_DigestVerifyInit(mdctx, nullptr, nullptr, nullptr, pubKey);

            int result = EVP_DigestVerify(mdctx, signature_.data(), signature_.size(), reinterpret_cast<const unsigned char*>(message.c_str()), message.size());

            EVP_MD_CTX_free(mdctx);
            EVP_PKEY_free(pubKey);
            return result == 1;        
        }

    private:
    std::string sender_;
    std::string receiver_;
    uint64_t amount_;
    std::string currency_;
    std::string txId_;
    std::vector<unsigned char> signature_;


        std::string computeId() const {
            std::string input = sender_ + receiver_ + std::to_string(amount_) + currency_;
            //generating txId just as the SHA256 Hash for the Block 
            unsigned char digest[SHA256_DIGEST_LENGTH];

            SHA256(reinterpret_cast<const unsigned char*>(input.c_str()), input.size(), digest);

            std::ostringstream oss;
            for (int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
                oss << std::hex << std:: setw(2) << std::setfill('0') << (int)digest[i];
            }

            return oss.str();
        }
};