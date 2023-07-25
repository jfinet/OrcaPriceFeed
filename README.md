# OrcaPriceFeed

Program to get the live price of the Orca whETH / USDc pool

There is dependencies hell conflicts for the borsh package between the Solana-sdk, anchor-lang, whirlpool, etc packages.
Let the cargo.lock as is.

Output example:

```
Orca whETH price: '1854.0603065460690692930984414' USDc
Orca whETH price: '1853.9088604601962707810415858' USDc
Orca whETH price: '1853.8762905775916554110154592' USDc
Orca whETH price: '1853.7249100174863197480582347' USDc
```

Ressources used:

- https://github.com/everlastingsong/solsandbox/blob/main/orca/whirlpool/rust_client/client_get_price/src/main.rs
- https://solanacookbook.com/references/local-development.html#connecting-to-environments
