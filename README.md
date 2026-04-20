# ✨ Stellar Wallet DApp (CRUD Version)

## 🧠 Overview

Stellar Wallet DApp adalah aplikasi dompet digital berbasis blockchain yang dibangun menggunakan Soroban SDK di jaringan Stellar. Aplikasi ini memungkinkan pengguna untuk menyimpan saldo, melakukan transaksi, dan mengelola riwayat keuangan secara desentralisasi.

Berbeda dengan aplikasi konvensional seperti DANA atau e-wallet lainnya, semua data disimpan langsung di blockchain sehingga:
- Tidak dapat dimanipulasi
- Transparan dan aman
- Dimiliki sepenuhnya oleh pengguna

---

## 🚀 Features (CRUD)

### 💰 Create (Top Up)
Menambahkan saldo ke dalam wallet secara langsung melalui smart contract.

### 📖 Read (View Balance & Transactions)
Melihat saldo dan seluruh riwayat transaksi secara real-time dari blockchain.

### 🔄 Update (Send Money)
Melakukan transfer saldo ke pengguna lain yang secara otomatis memperbarui saldo dan mencatat transaksi.

### ❌ Delete (Delete Transaction)
Menghapus riwayat transaksi tertentu berdasarkan ID.

---

## 🔐 Security (Like DANA)

- Menggunakan autentikasi wallet (`require_auth`)
- Sistem berbasis ownership
- Transaksi hanya dapat dilakukan oleh pemilik akun
- Data tidak dapat dimodifikasi oleh pihak lain

---

## 📦 Data Structure

Transaction {
  id: u64,
  from: String,
  to: String,
  amount: u64
}

Balance {
  amount: u64
}

---

## ⚙️ Smart Contract Functions

- top_up → Menambah saldo
- get_balance → Melihat saldo
- send_money → Mengirim uang
- get_transactions → Melihat riwayat transaksi
- delete_transaction → Menghapus transaksi

---

## 🔗 Contract Address

# ✨ Stellar Wallet DApp (CRUD Version)

## 🧠 Overview

Stellar Wallet DApp adalah aplikasi dompet digital berbasis blockchain yang dibangun menggunakan Soroban SDK di jaringan Stellar. Aplikasi ini memungkinkan pengguna untuk menyimpan saldo, melakukan transaksi, dan mengelola riwayat keuangan secara desentralisasi.

Berbeda dengan aplikasi konvensional seperti DANA atau e-wallet lainnya, semua data disimpan langsung di blockchain sehingga:
- Tidak dapat dimanipulasi
- Transparan dan aman
- Dimiliki sepenuhnya oleh pengguna

---

## 🚀 Features (CRUD)

### 💰 Create (Top Up)
Menambahkan saldo ke dalam wallet secara langsung melalui smart contract.

### 📖 Read (View Balance & Transactions)
Melihat saldo dan seluruh riwayat transaksi secara real-time dari blockchain.

### 🔄 Update (Send Money)
Melakukan transfer saldo ke pengguna lain yang secara otomatis memperbarui saldo dan mencatat transaksi.

### ❌ Delete (Delete Transaction)
Menghapus riwayat transaksi tertentu berdasarkan ID.

---

## 🔐 Security (Like DANA)

- Menggunakan autentikasi wallet (`require_auth`)
- Sistem berbasis ownership
- Transaksi hanya dapat dilakukan oleh pemilik akun
- Data tidak dapat dimodifikasi oleh pihak lain

---

## 📦 Data Structure

Transaction {
  id: u64,
  from: String,
  to: String,
  amount: u64
}

Balance {
  amount: u64
}

---

## ⚙️ Smart Contract Functions

- top_up → Menambah saldo
- get_balance → Melihat saldo
- send_money → Mengirim uang
- get_transactions → Melihat riwayat transaksi
- delete_transaction → Menghapus transaksi

---

## 🔗 Contract Address

CCCQJ3X67EAQWGAPICI5GALWMMM4EHTD4PWQNILDACZBO2WPXMV3GIF7

---

## 🛠️ Tech Stack

- Rust
- Soroban SDK
- Stellar Blockchain

---

## ⚡ How to Use

Top Up Saldo:
stellar contract invoke --id CONTRACT_ID --source alice --network testnet -- top_up --amount 1000

Cek Saldo:
stellar contract invoke --id CONTRACT_ID --source alice --network testnet -- get_balance

Kirim Uang:
stellar contract invoke --id CONTRACT_ID --source alice --network testnet -- send_money --to "Bintang" --amount 200

Lihat Transaksi:
stellar contract invoke --id CONTRACT_ID --source alice --network testnet -- get_transactions

Hapus Transaksi:
stellar contract invoke --id CONTRACT_ID --source alice --network testnet -- delete_transaction --id 123

---

## 🌍 Vision

- Menggantikan sistem e-wallet terpusat
- Memberikan kontrol penuh kepada pengguna terhadap saldo mereka
- Menyediakan sistem keuangan yang transparan dan trustless
- Meningkatkan keamanan data finansial dengan blockchain

---

## 🔮 Future Development

- UI Web / Mobile seperti aplikasi DANA
- Sistem multi-user dengan address-based account
- Enkripsi data transaksi
- Integrasi dengan token Stellar (XLM / custom token)
- Fitur notifikasi transaksi
- Analisis keuangan berbasis AI

---

## 📌 Conclusion

Stellar Wallet DApp adalah simulasi dompet digital berbasis blockchain yang memungkinkan pengguna untuk mengelola saldo dan transaksi secara aman, transparan, dan tanpa bergantung pada sistem terpusat.

---

## 🛠️ Tech Stack

- Rust
- Soroban SDK
- Stellar Blockchain

---

## ⚡ How to Use

Top Up Saldo:
stellar contract invoke --id CONTRACT_ID --source alice --network testnet -- top_up --amount 1000

Cek Saldo:
stellar contract invoke --id CONTRACT_ID --source alice --network testnet -- get_balance

Kirim Uang:
stellar contract invoke --id CONTRACT_ID --source alice --network testnet -- send_money --to "Bintang" --amount 200

Lihat Transaksi:
stellar contract invoke --id CONTRACT_ID --source alice --network testnet -- get_transactions

Hapus Transaksi:
stellar contract invoke --id CONTRACT_ID --source alice --network testnet -- delete_transaction --id 123

---

## 🌍 Vision

- Menggantikan sistem e-wallet terpusat
- Memberikan kontrol penuh kepada pengguna terhadap saldo mereka
- Menyediakan sistem keuangan yang transparan dan trustless
- Meningkatkan keamanan data finansial dengan blockchain

---

## 🔮 Future Development

- UI Web / Mobile seperti aplikasi DANA
- Sistem multi-user dengan address-based account
- Enkripsi data transaksi
- Integrasi dengan token Stellar (XLM / custom token)
- Fitur notifikasi transaksi
- Analisis keuangan berbasis AI

---

## 📌 Conclusion

Stellar Wallet DApp adalah simulasi dompet digital berbasis blockchain yang memungkinkan pengguna untuk mengelola saldo dan transaksi secara aman, transparan, dan tanpa bergantung pada sistem terpusat.