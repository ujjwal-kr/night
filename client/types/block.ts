import { Transaction } from "./transaction"

export interface Block {
    id: Number
    previous_hash: String
    transaction: Transaction
    block_hash: String
}