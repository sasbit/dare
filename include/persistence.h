#pragma once
#include <fstream>
#include <string>
#include "json.hpp"
#include "blockchain.h"

inline void saveChain(const Blockchain& bc) {
    using json = nlohmann::json;
    json j = json::array();

    for (const Block& b : bc.getChain()) {
        json blockJson;
        blockJson["index"] = b.getIndex();
        blockJson["timestamp"] = b.getTimestamp();
        blockJson["previousHash"] = b.getPreviousHash();
        blockJson["hash"] = b.getHash();

        json txArray = json::array();
        for (const Transaction& tx : b.getTransactions()) {
            json txJson;
            txJson["sender"] = tx.getSender();
            txJson["receiver"] = tx.getReceiver();
            txJson["amount"] = tx.getAmount();
            txJson["currency"] = tx.getCurrency();
            txJson["txId"] = tx.getTxId();
            //adding the finished txJson for each transaction to the end of the txArray
            txArray.push_back(txJson);
        }
        //adding the txArray for a Block to the blockJson
        blockJson["transactions"] = txArray;
        //adding the finished blockJson to json array j that will contain all blocks and their transactions
        j.push_back(blockJson);
    }

    std::ofstream file("chain.json");
    file << j.dump(4);
}

inline Blockchain loadChain() {
    using json = nlohmann::json;
    Blockchain bc(Blockchain::LoadTag{});
    std::ifstream file("chain.json");
    if (!file.is_open()) {
        return bc;
    }

    json j = json::parse(file);

    for (const auto& blockJson : j) {
        std::vector<Transaction> txs;
        for (const auto& txJson : blockJson["transactions"]) {
            txs.emplace_back(
                txJson["sender"].get<std::string>(),
                txJson["receiver"].get<std::string>(),
                txJson["amount"].get<uint64_t>(),
                txJson["currency"].get<std::string>()
            );
        }
        bc.loadBlock(blockJson["index"],
            blockJson["timestamp"],
            txs,
            blockJson["previousHash"],
            blockJson["hash"]);
    }
    return bc;
}