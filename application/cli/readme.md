
```
cargo run --bin=client -- --solana_rpc_url=https://api.devnet.solana.com initialize --intermediary_investor=/root/.config/solana/id.json --intermediary_manager=ToFrfRoptxjydn9wtuTJKm9WpxLsqXqReMzuoPs3vgV --intermediary_trader=YztkfMR7NunaNW1rtpXp7WPunWT1NyGB1m41KoDv5yN --lamports_to_treasury=1021
```
```
cargo run --bin=client -- --solana_rpc_url=https://api.devnet.solana.com deposit_funds --intermediary_investor=/root/.config/solana/id.json --intermediary=? --lamports_to_treasury=123
```
```
cargo run --bin=client -- --solana_rpc_url=https://api.devnet.solana.com withdraw_funds --intermediary_investor=/root/.config/solana/id.json --intermediary=? --lamports_from_treasury=123
```
```
cargo run --bin=client -- --solana_rpc_url=https://api.devnet.solana.com change_manager --intermediary_investor=/root/.config/solana/id.json --intermediary=? --intermediary_manager=?
```
```
cargo run --bin=client -- --solana_rpc_url=https://api.devnet.solana.com change_trader --intermediary_manager=? --intermediary=? --intermediary_trader=?
```

DEVNET:
intermediary_investor (4ucJUDCdw7NNBcKwJsbWWr1pSw8piXQZiBHuH1A7GvoX)
[89,244,210,114,239,117,114,250,216,23,202,32,32,180,31,233,235,78,106,57,202,107,60,229,220,234,20,34,235,73,64,232,58,13,45,147,199,39,193,205,80,3,25,52,26,250,233,91,79,120,168,151,20,103,102,196,36,224,123,248,18,67,136,86]
intermediary_manager (ToFrfRoptxjydn9wtuTJKm9WpxLsqXqReMzuoPs3vgV)
[18,71,1,68,16,34,145,74,87,57,54,49,38,97,13,17,93,3,42,173,154,80,149,180,34,38,51,45,122,156,150,128,6,221,103,44,127,146,81,198,179,27,234,103,42,164,6,137,40,253,236,176,142,147,255,255,226,32,97,170,231,198,53,22]
intermediary_trader (YztkfMR7NunaNW1rtpXp7WPunWT1NyGB1m41KoDv5yN)
[181,209,210,191,93,68,191,227,149,66,242,125,153,36,195,88,181,16,155,162,80,227,88,94,126,161,182,209,115,228,207,142,8,50,118,207,91,186,17,131,26,36,58,205,79,206,145,200,80,164,191,143,197,74,178,227,69,118,50,141,39,41,22,157]
