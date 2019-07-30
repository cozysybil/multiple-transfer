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
- **client_proxy.rs** (modified) line 415 - 489 : add new function name "multi_transfer_coins"  
- **commands.rs** (modified) line 8 : add crate name "MultipleTransferCommand", line 67 : add new command name "MultipleTransferCommand"  
- **libs.rs** (modified) line 24 : add new wrapper "multiple_transfer_commands"          
- (new file) **multiple_transfer_commands.rs** 
