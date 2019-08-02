# multiple-transfer
## my two ways for do multiple transfer transaction in libra
the 1st one I try to edit the CLI to add new CLI-commands **client/src/** that can transfer libra coins from 1 to many accounts(max=10 accounts)  
the 2nd is the transaction script (.mvir) **test/** use for transfer libra coins from 1 to **only 3** accounts that you must build the transaction before use "submitb" in CLI to submit it to your local chain   
read more about transaction script at https://developers.libra.org/docs/move-overview#writing-transaction-scripts
