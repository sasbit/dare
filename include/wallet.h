#pragma once
#include <openssl/evp.h>
#include <vector>
#include <string>
#include <stdexcept>

class Wallet {
    public: 
        Wallet() {
            EVP_PKEY_CTX* ctx =EVP_PKEY_CTX_new_id(EVP_PKEY_ED25519, nullptr);
            EVP_PKEY_keygen_init(ctx);
            EVP_PKEY_keygen(ctx, &key_);
            EVP_PKEY_CTX_free(ctx);
        }

        ~Wallet() { EVP_PKEY_free(key_); }

        Wallet(const Wallet&) = delete;
        Wallet& operator=(const Wallet&) = delete;

        std::vector<unsigned char> sign(const std::string& message) const {
            EVP_MD_CTX* mdctx = EVP_MD_CTX_new();
            EVP_DigestSignInit(mdctx, nullptr, nullptr, nullptr, key_);

            size_t siglen;
            EVP_DigestSign(mdctx, nullptr, &siglen, reinterpret_cast<const unsigned char*>(message.c_str()), message.size());

            std::vector<unsigned char> sig(siglen);
            EVP_DigestSign(mdctx, sig.data(), &siglen, reinterpret_cast<const unsigned char*>(message.c_str()), message.size());

            EVP_MD_CTX_free(mdctx);
            return sig;
        }

        std:: vector<unsigned char> getPublicKey() const {
            size_t len = 32;
            std::vector<unsigned char> pub(len);
            EVP_PKEY_get_raw_public_key(key_, pub.data(), &len);
            return pub;
        }

        private:
            EVP_PKEY* key_ = nullptr;
};