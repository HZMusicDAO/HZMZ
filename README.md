# Solana HZMZ NFT Minting Project

## Prerequisites
   Please follow this installation instruction.
   https://book.anchor-lang.com/getting_started/installation.html

## How to deploy
    git clone https://github.com/HZMusicDAO/HZMZ

    cd HZMZ

    npm install

    anchor build

    solana address -k target/deploy/solana_nft-keypair.json
   

And then you can get the address. This is program address.
You should copy it and then paste the exact place of the Anchor.toml and programs/solana_nft/src/lib.rs.

Anchor.toml

    [programs.localnet]
    solana_nft = "B9n6kvLkZGATZCvL6H2ujjzQE28jvbT2k6N1vKt9hC6L"

    [programs.devnet]
    solana_nft = "B9n6kvLkZGATZCvL6H2ujjzQE28jvbT2k6N1vKt9hC6L"

    [programs.testnet]
    solana_nft = "B9n6kvLkZGATZCvL6H2ujjzQE28jvbT2k6N1vKt9hC6L"

    [programs.mainnet]
    solana_nft = "B6ZwLmyVJvsMZW8ET6Zr6VohM4xptTtsMByJdybL1cqv"

programs/solana_nft/src/lib.rs

    declare_id!("BZmmzuyipMWmFPaXaEf8kD9nm7SuLhmQ68B4nQ3xEzEa");

Finally please deploy the smart contract with the following:

    anchor build
    anchor deploy
