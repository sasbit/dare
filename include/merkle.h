#pragma once
#include <openssl/sha.h>
#include <sstream>
#include <iomanip>
#include <vector>
#include <string>

static std::string hashPair(const std::string& a, const std::string& b) {
    std::string input = a + b;
    
    unsigned char digest[SHA256_DIGEST_LENGTH];
    SHA256(reinterpret_cast<const unsigned char*>(input.c_str()), input.size(), digest);

    std::ostringstream oss;
    for (int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
        oss << std::hex << std::setw(2) << std::setfill('0') << (int)digest[i];
    }

    return oss.str();

}

inline std::string computeMerkleRoot(std::vector<std::string> hashes) {
    if (hashes.empty()) {
        return std::string(64, '0');
    }

    if (hashes.size() == 1) {
        return hashes[0];
    }

    //if odd number of hashes, duplicate the last one

    if (hashes.size() % 2 != 0 ) {
        hashes.push_back(hashes.back());
    }

    std::vector<std::string> nextLevel;
    for (size_t i = 0; i < hashes.size(); i+=2) {
        nextLevel.push_back(hashPair(hashes[i], hashes[i+1]));
    }

    return computeMerkleRoot(nextLevel); //recursing up the tree
}