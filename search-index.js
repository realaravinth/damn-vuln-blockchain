var searchIndex = JSON.parse('{\
"damn_vuln_blockchain":{"doc":"This is a test blockchain that I build for fun and as the…","i":[[0,"asset","damn_vuln_blockchain","",null,null],[3,"Asset","damn_vuln_blockchain::asset","",null,null],[3,"AssetLedger","","",null,null],[12,"assets","","",0,null],[11,"new","","create new asset",1,[[]]],[11,"get_name","","get name of the asset",1,[[]]],[11,"get_hash","","get hash of the asset",1,[[]]],[11,"get_value","","get value/price of the asset",1,[[]]],[11,"set_owner","","set owner of the asset",1,[[]]],[11,"get_owner","","get owner of the asset",1,[[],["option",4]]],[11,"generate","","",0,[[],["assetledger",3]]],[0,"blockchain","damn_vuln_blockchain","",null,null],[3,"BlockBuilder","damn_vuln_blockchain::blockchain","",null,null],[3,"Block","","Block. `tx`, `prev`, validator and `rx` are `Option<_>` to…",null,null],[3,"Chain","","",null,null],[11,"set_prev","","set previous block\'s hash",2,[[["block",3]]]],[11,"set_rx","","set receiver\'s ID",2,[[]]],[11,"set_tx","","set sender\'s ID",2,[[]]],[11,"set_validator","","set validator\'s ID",2,[[]]],[11,"set_asset_id","","set assset ID",2,[[["asset",3]]]],[11,"build","","Build block, this method must be called at the very end",2,[[],["block",3]]],[11,"to_string","","Get block info as string",3,[[],["string",3]]],[11,"is_genesis","","checks if the block is a genesis block",3,[[]]],[11,"hash","","computes the hash of a block, uses the same logic for…",3,[[],["string",3]]],[11,"get_prev","","get hash of previous block",3,[[],[["string",3],["option",4]]]],[11,"get_hash","","get hash of block",3,[[]]],[11,"get_rx","","get receiver involved in the transaction that lead tot the…",3,[[],[["string",3],["option",4]]]],[11,"get_validator","","get validator involved in the creation of this block",3,[[],[["string",3],["option",4]]]],[11,"get_tx","","get sender involved in the transaction that lead tot the…",3,[[],[["string",3],["option",4]]]],[11,"new","","create new blockchain",4,[[],["chain",3]]],[11,"get_last_block","","get the last block in the chain",4,[[],["block",3]]],[11,"add_block","","add a block to the chain…",4,[[["block",3]],["chainresult",6]]],[11,"is_valid","","checks if a blockchain is valid by comparing the hash of…",4,[[],["chainresult",6]]],[0,"error","damn_vuln_blockchain","",null,null],[4,"ChainError","damn_vuln_blockchain::error","Errors that can occur when interacting with the blockchain",null,null],[13,"GenesisBlockAdditionError","","when the a genesis block is passed to get added to a chain…",5,null],[13,"InvalidBlockChain","","Blockchain invalid when block.hash() != block.next().prev",5,null],[6,"ChainResult","","",null,null],[0,"logs","damn_vuln_blockchain","",null,null],[3,"LoggableAction","damn_vuln_blockchain::logs","",null,null],[12,"timesamp","","",6,null],[12,"action","","",6,null],[3,"Peer","","",null,null],[12,"id","","",7,null],[12,"balance","","",7,null],[3,"Gossip","","",null,null],[12,"tx","","",8,null],[12,"rx","","",8,null],[4,"Action","","",null,null],[13,"InitLog","","Initialize log",9,null],[13,"PeerConnected","","Peer connected event, pass in peer ID",9,null],[13,"MintingAsset","","Miniting asset event, pass in Asset ID",9,null],[13,"DistributingAssets","","Distributing asset `asset ID` to peer `peer  ID`",9,null],[13,"TransactionRequest","","Transaction request event from peer `peer ID` for asset…",9,null],[13,"StakeBroadcast","","Stake broadcast event from peer `peer ID` with stake",9,null],[13,"TransactionValidated","","Transaction validation event. Transaction validated by…",9,null],[13,"BlockCreation","","Block creation event for block `block ID`",9,null],[13,"TransactionBroadcasting","","Transaction broadcasting event by peer `peer ID` to peer…",9,null],[11,"new","","",6,[[["action",4]],["loggableaction",3]]],[0,"utils","damn_vuln_blockchain","",null,null],[5,"hasher","damn_vuln_blockchain::utils","helper function for generating sha256 hashes",null,[[],["string",3]]],[5,"get_rand_string","","helper function for generating random strings of length =…",null,[[],["string",3]]],[5,"get_current_time","","helper function to get current timesamp",null,[[],["string",3]]],[11,"from","damn_vuln_blockchain::asset","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"to_string","","",1,[[],["string",3]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"from","damn_vuln_blockchain::blockchain","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"to_string","","",3,[[],["string",3]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"vzip","","",4,[[]]],[11,"from","damn_vuln_blockchain::error","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_owned","","",5,[[]]],[11,"clone_into","","",5,[[]]],[11,"to_string","","",5,[[],["string",3]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"vzip","","",5,[[]]],[11,"from","damn_vuln_blockchain::logs","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_string","","",6,[[],["string",3]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"vzip","","",6,[[]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"to_owned","","",7,[[]]],[11,"clone_into","","",7,[[]]],[11,"to_string","","",7,[[],["string",3]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"vzip","","",7,[[]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"to_owned","","",8,[[]]],[11,"clone_into","","",8,[[]]],[11,"to_string","","",8,[[],["string",3]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"vzip","","",8,[[]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"to_string","","",9,[[],["string",3]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"vzip","","",9,[[]]],[11,"clone","damn_vuln_blockchain::asset","",1,[[],["asset",3]]],[11,"clone","","",0,[[],["assetledger",3]]],[11,"clone","damn_vuln_blockchain::blockchain","",2,[[],["blockbuilder",3]]],[11,"clone","","",3,[[],["block",3]]],[11,"clone","","",4,[[],["chain",3]]],[11,"clone","damn_vuln_blockchain::error","",5,[[],["chainerror",4]]],[11,"clone","damn_vuln_blockchain::logs","",7,[[],["peer",3]]],[11,"clone","","",8,[[],["gossip",3]]],[11,"default","damn_vuln_blockchain::blockchain","",2,[[],["blockbuilder",3]]],[11,"default","","",3,[[],["block",3]]],[11,"default","damn_vuln_blockchain::logs","",7,[[],["peer",3]]],[11,"default","","",8,[[],["gossip",3]]],[11,"eq","damn_vuln_blockchain::error","",5,[[["chainerror",4]]]],[11,"fmt","damn_vuln_blockchain::asset","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::blockchain","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","","",4,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::error","",5,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::logs","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::asset","",1,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::blockchain","",3,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::error","",5,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::logs","",6,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"serialize","damn_vuln_blockchain::asset","",1,[[],["result",4]]],[11,"serialize","","",0,[[],["result",4]]],[11,"serialize","damn_vuln_blockchain::blockchain","",2,[[],["result",4]]],[11,"serialize","","",3,[[],["result",4]]],[11,"serialize","","",4,[[],["result",4]]],[11,"serialize","damn_vuln_blockchain::logs","",7,[[],["result",4]]],[11,"serialize","","",8,[[],["result",4]]],[11,"deserialize","damn_vuln_blockchain::asset","",1,[[],["result",4]]],[11,"deserialize","","",0,[[],["result",4]]],[11,"deserialize","damn_vuln_blockchain::blockchain","",2,[[],["result",4]]],[11,"deserialize","","",3,[[],["result",4]]],[11,"deserialize","","",4,[[],["result",4]]],[11,"deserialize","damn_vuln_blockchain::logs","",7,[[],["result",4]]],[11,"deserialize","","",8,[[],["result",4]]]],"p":[[3,"AssetLedger"],[3,"Asset"],[3,"BlockBuilder"],[3,"Block"],[3,"Chain"],[4,"ChainError"],[3,"LoggableAction"],[3,"Peer"],[3,"Gossip"],[4,"Action"]]},\
"dwb":{"doc":"","i":[[3,"Config","dwb","",null,null],[12,"auditor","","",0,null],[12,"peer_id","","",0,null],[12,"port","","",0,null],[5,"main","","",null,[[],["result",6]]],[5,"cli","","",null,[[],["config",3]]],[0,"routes","","",null,null],[3,"greet","dwb::routes","",null,null],[5,"services","","",null,[[["serviceconfig",3]]]],[11,"from","dwb","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"from","dwb::routes","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"clone","dwb","",0,[[],["config",3]]],[11,"register","dwb::routes","",1,[[["appservice",3]]]]],"p":[[3,"Config"],[3,"greet"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);