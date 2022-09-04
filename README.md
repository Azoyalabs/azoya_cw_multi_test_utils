# azoya_cw_multi_test_utils 

Wrapping some convenience functions for cw_multi_test. 
Current utils available: 
- Mint native currency
- Advance block number and time  

```rust 
pub trait TimeTravel {
    fn advance_time(&mut self, nb_blocks_to_advance: u64, nb_seconds_to_advance: u64);
}

pub trait CashMachine {
    fn mint_native(&mut self, beneficiary: &str, to_mint: Coin);
}
```
