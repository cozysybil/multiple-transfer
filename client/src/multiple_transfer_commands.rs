// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{client_proxy::ClientProxy, commands::*};


/// Command to transfer coins from one to two accounts.
pub struct MultipleTransferCommand {}

impl Command for MultipleTransferCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["multransfer", "mt"]
    }
    fn get_params_help(&self) -> &'static str {
        "\n\t<sender_account_address> <number_of_coins> \
         [gas_unit_price_in_micro_libras (0)] [max_gas_amount_in_micro_libras (100000)] \
         <number of reciever (max=10)> [list of <receiver_account_address>] \
         "
    }
    fn get_description(&self) -> &'static str {
        "Transfer coins (in libra) from one to many accounts."
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() < 7 || params.len() > 15 {
            println!("Invalid number of arguments for transfer");
            println!(
                "{} {}",
                self.get_aliases().join(" | "),
                self.get_params_help()
            );
            return;
        }
    
 
        println!(">> Transferring");
        let is_blocking = blocking_cmd(&params[0]); 
        match client.multi_transfer_coins(&params, is_blocking) {
            Ok(_) => {
                if is_blocking {
                    println!("Finished transaction!");
                } else {
                    println!("Transaction submitted to validator");
                }
            },
            Err(e) => report_error("Failed to perform transaction", e),
        }
    }
}
