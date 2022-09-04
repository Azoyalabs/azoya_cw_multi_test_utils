use cosmwasm_std::Coin;
use cw_multi_test::{App, BankSudo, SudoMsg};

pub trait TimeTravel {
    fn advance_time(&mut self, nb_blocks_to_advance: u64, nb_seconds_to_advance: u64);
}

impl TimeTravel for App {
    fn advance_time(&mut self, nb_blocks_to_advance: u64, nb_seconds_to_advance: u64) {
        let mut block_info = self.block_info();
        block_info.height += nb_blocks_to_advance;
        block_info.time = block_info.time.plus_seconds(nb_seconds_to_advance);

        self.set_block(block_info);
    }
}

pub trait CashMachine {
    fn mint_native(&mut self, beneficiary: &str, to_mint: Coin);
}

impl CashMachine for App {
    fn mint_native(&mut self, beneficiary: &str, to_mint: Coin) {
        self.sudo(SudoMsg::Bank(BankSudo::Mint {
            to_address: beneficiary.to_string(),
            amount: vec![to_mint],
        }))
        .unwrap();
    }
}
