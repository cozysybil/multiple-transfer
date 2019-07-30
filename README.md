# multiple-transfer
## How to use the command
multiple-transfer use for send coin from one to n account equally.
```
multransfer | mt 
	<sender_account_address> <number_of_coins> [gas_unit_price_in_micro_libras (0)] [max_gas_amount_in_micro_libras (100000)] <number of reciever (max=10)> [list of <receiver_account_address>]
```
example
```
libra% multransfer 0 1000 0 100000 5 1 2 3 4 3465672fad49fc5be7db1040a9d0840e3676a5479f1f4bd63505806dd21d3e60
```
the above command means account0 send 1000 libra coins to 5 accounts:  
- account1,  
- account2,  
- account3,  
- account4 and   
- "3465672fad49fc5be7db1040a9d0840e3676a5479f1f4bd63505806dd21d3e60"  

with 1000 / 5 = 200 libra for each account

you can use number or address for <receiver_account_address>

## file directory
libra/client/src/  
1. **client_proxy.rs** (modified) line 415 - 489 : add new function name "multi_transfer_coins"  
2. **commands.rs** (modified) line 8 : add crate name "MultipleTransferCommand", line 67 : add new command name "MultipleTransferCommand"  
3. **libs.rs** (modified) line 24 : add new wrapper "multiple_transfer_commands"          
4. (new file) **multiple_transfer_commands.rs**  

replace all "modified" file and add new "multiple_transfer_commands.rs" to the directory. then run libra_swarm in libra/  
```
cargo run -p libra_swarm -- -s
```
don't forget to edit the "/libra/terraform/validator-sets/dev/node.config.toml" at  
**[vm_config][vm_config.publishing_options] type = "Lock"**  to  
**[vm_config][vm_config.publishing_options] type = "Open"**
