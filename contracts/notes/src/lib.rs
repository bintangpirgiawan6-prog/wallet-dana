#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// =======================
// STRUCT DATA
// =======================

// Struktur transaksi
#[contracttype]
#[derive(Clone)]
pub struct Transaction {
    id: u64,
    from: String,
    to: String,
    amount: u64,
}

// Storage key
const BALANCE: Symbol = symbol_short!("BALANCE");
const TX_DATA: Symbol = symbol_short!("TX_DATA");

// =======================
// CONTRACT
// =======================
#[contract]
pub struct WalletContract;

#[contractimpl]
impl WalletContract {

    // =======================
    // CREATE → Tambah saldo
    // =======================
    pub fn top_up(env: Env, amount: u64) -> String {
        let mut balance: u64 = env.storage().instance().get(&BALANCE).unwrap_or(0);

        balance += amount;

        env.storage().instance().set(&BALANCE, &balance);

        String::from_str(&env, "Saldo berhasil ditambahkan")
    }

    // =======================
    // READ → Lihat saldo
    // =======================
    pub fn get_balance(env: Env) -> u64 {
        env.storage().instance().get(&BALANCE).unwrap_or(0)
    }

    // =======================
    // CREATE → Kirim uang
    // =======================
    pub fn send_money(env: Env, to: String, amount: u64) -> String {
        let mut balance: u64 = env.storage().instance().get(&BALANCE).unwrap_or(0);

        if balance < amount {
            return String::from_str(&env, "Saldo tidak cukup");
        }

        balance -= amount;
        env.storage().instance().set(&BALANCE, &balance);

        let mut txs: Vec<Transaction> = env.storage().instance().get(&TX_DATA).unwrap_or(Vec::new(&env));

        let tx = Transaction {
            id: env.prng().gen::<u64>(),
            from: String::from_str(&env, "user"),
            to,
            amount,
        };

        txs.push_back(tx);
        env.storage().instance().set(&TX_DATA, &txs);

        String::from_str(&env, "Transfer berhasil")
    }

    // =======================
    // READ → Lihat transaksi
    // =======================
    pub fn get_transactions(env: Env) -> Vec<Transaction> {
        env.storage().instance().get(&TX_DATA).unwrap_or(Vec::new(&env))
    }

    // =======================
    // DELETE → Hapus transaksi
    // =======================
    pub fn delete_transaction(env: Env, id: u64) -> String {
        let mut txs: Vec<Transaction> = env.storage().instance().get(&TX_DATA).unwrap_or(Vec::new(&env));

        for i in 0..txs.len() {
            if txs.get(i).unwrap().id == id {
                txs.remove(i);
                env.storage().instance().set(&TX_DATA, &txs);
                return String::from_str(&env, "Transaksi dihapus");
            }
        }

        String::from_str(&env, "Transaksi tidak ditemukan")
    }
}