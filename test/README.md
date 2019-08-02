# multiple-transfer
## How to use the .mvir
multiple-transfer use for send coin from one to 3 account equally.  

when you have **transaction_script(.mvir)**, you can use **compiler** to compile it to the **.program** file. then use **transaction_builder** with **arguments** for build it to **.txn**, and use CLI for submit the .txn with "sender" of transaction.
  
**.mvir** --> |compiler| --> **.program** --> |transaction_builder|+<args>  --> **.txn**  
  
### terminal 1
cd to **libra/** then run...  
```
cargo run -p libra_swarm -- -s
```
1. create 4 accounts (aacount0, account1, account2, account3)  
2. mint **account0** for 10 Libra (up to you!!)  
3. use **account0** transfer 1 libra to **account1** (up to you!!) (because **seq.no of sender must more than 0**)

### terminal 2
cd to **libra/** then run below cmd for create .program from mvir,...
```
compiler -o <output_program> <transaction_script>
```
for example...
```
./target/debug/compiler -o ./test/multi_transfer_v1.program ./test/multi_transfer_u64.mvir
```  
I use compiler in "libra/target/debug/" for compiling "libra/test/**multi_transfer_u64.mvir**" to "libra/test/**multi_transfer_v1.program**"
then use **address of account0, account1, account2, account3** and **seq.no of account0** for 'args' when build the transaction...
```
transaction_builder <sender_address> <sender_sequence_no> <program> <output_transaction> --args <amount> 0x<receiver1> 0x<receiver3> 0x<receiver2>
```
for example...
```
./target/debug/transaction_builder c79609e93e03dfa31211c2606e67b5de62d4fb43205bd84bd947f9a9e6c8f849 1 ./test/multi_transfer3.program out1.txn --args 3000000 0x34fdf4025cfacbf1d321372e9e2ac8d659c85b4c91758fdc312b74d15133fda9 0xbbdfe440aaa448badbc983e8900d8b88d9e11b60cfd544069298e60f6bdd5286 0x916589e91c9b944ec3df8346c6c1ec7b825bd09606ec20999e9d3e69b1de952e
```
I use builder in "libra/target/debug/" for building "libra/test/**multi_transfer3.program**" to "libra/out1.txn" with account0_address as a sender with sequence number "1" and use "libra in micro unit" and 0x"account_address" for the argument.
and then use .txn for submit transaction, back to terminal1...  

### terminal 1
use **dev submitb** for submit transaction like this...
```
libra% dev sb <sender> <transaction>
```
for example
```
libra% dev sb 0 out1.txn
```
you will get...
```
[waiting transaction is stored!
Finished transaction!
To query for transaction status, run: query txn_acc_seq c79609e93e03dfa31211c2606e67b5de62d4fb43205bd84bd947f9a9e6c8f849 1 <fetch_events=true|false>
```
then you can query balance to check the result etc.

## file directory
libra/test/  
1. **multi_transfer_u64.mvir** withdraw amount/3 from sender 3 times to pay for each account  
2. **multi_transfer_resource.mvir** withdraw amount from sender just one time then move devided coin out of the amount  

don't need to edit any ".toml"
