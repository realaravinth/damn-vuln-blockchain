var searchIndex = JSON.parse('{\
"damn_vuln_blockchain":{"doc":"This is a test blockchain that I build for fun and as the…","i":[[0,"asset","damn_vuln_blockchain","Assets are objects that can be transacted on the blockchain",null,null],[3,"Asset","damn_vuln_blockchain::asset","/// [Asset]s are objects that can be transacted on the…",null,null],[3,"AssetLedger","","represents the world(full network) state of [Asset]",null,null],[12,"assets","","",0,null],[12,"stake","","",0,null],[12,"peer_id","","",0,null],[3,"Stake","","represents the stake each peer is willing to send for…",null,null],[12,"block_id","","",1,null],[12,"stake","","",1,null],[3,"StakeBuilder","","Builder for `Stake`.",null,null],[3,"ChangeAssetOwner","","Changes owner of asset `ChangeOwner.0` to  `ChangeOwner.1`",null,null],[12,"asset_id","","",2,null],[12,"new_owner","","",2,null],[3,"ChangeAssetOwnerBuilder","","Builder for `ChangeAssetOwner`.",null,null],[3,"InitNetwork","","Initializes assets to peers in the network AssetLedger`…",null,null],[12,"network_size","","",3,null],[12,"peer_id","","",3,null],[3,"InitNetworkBuilder","","Builder for `InitNetwork`.",null,null],[3,"GetAssetInfo","","Get asset info of `GetAssetInfo.0`",null,null],[12,"0","","",4,null],[3,"DumpLedger","","Dumps entire ledger",null,null],[3,"ReplaceLedger","","Replaces asset ledger Useful when forking",null,null],[12,"0","","",5,null],[3,"ChooseValidator","","Get asset info of `GetAssetInfo.0`",null,null],[3,"GetPeerAssets","","Get assets belonging to a peer",null,null],[12,"0","","",6,null],[3,"SetStake","","Set stake for a block ID",null,null],[12,"block_id","","",7,null],[12,"stake","","",7,null],[12,"peer_id","","",7,null],[3,"SetStakeBuilder","","Builder for `SetStake`.",null,null],[3,"GetStake","","Get stake for a block ID",null,null],[12,"0","","",8,null],[11,"new","","create new asset",9,[[]]],[11,"get_name","","get name of the asset",9,[[]]],[11,"get_hash","","get hash of the asset",9,[[]]],[11,"get_value","","get value/price of the asset",9,[[]]],[11,"set_owner","","set owner of the asset",9,[[]]],[11,"set_last_transaction","","set last transaction validated by the asset",9,[[]]],[11,"get_owner","","get owner of the asset",9,[[],["option",4]]],[11,"get_last_transaction","","get last transaction validated by the asset",9,[[]]],[11,"block_id","","",10,[[]]],[11,"stake","","",10,[[["vec",3],["string",3]]]],[11,"build","","Builds a new `Stake`.",10,[[],[["result",4],["stake",3],["string",3]]]],[11,"generate","","generates a bunch of fake assets",0,[[],["assetledger",3]]],[11,"choose_validator","","chooses validator based on proof of stake",0,[[],[["option",4],["string",3]]]],[11,"asset_id","","",11,[[["string",3]]]],[11,"new_owner","","",11,[[["string",3]]]],[11,"build","","Builds a new `ChangeAssetOwner`.",11,[[],[["changeassetowner",3],["string",3],["result",4]]]],[11,"network_size","","",12,[[]]],[11,"peer_id","","",12,[[["string",3]]]],[11,"build","","Builds a new `InitNetwork`.",12,[[],[["string",3],["result",4],["initnetwork",3]]]],[11,"block_id","","",13,[[]]],[11,"stake","","",13,[[["vec",3],["string",3]]]],[11,"peer_id","","",13,[[["string",3]]]],[11,"build","","Builds a new `SetStake`.",13,[[],[["result",4],["setstake",3],["string",3]]]],[0,"block","damn_vuln_blockchain","the smallest unit/data-structure that can go into the…",null,null],[3,"BlockBuilder","damn_vuln_blockchain::block","Builder struct for [Block]",null,null],[3,"Block","","Smallest data-sctructure that can go into Chain.",null,null],[11,"set_prev","","set previous block\'s hash",14,[[["block",3]]]],[11,"set_rx","","set receiver\'s ID",14,[[]]],[11,"set_tx","","set sender\'s ID",14,[[]]],[11,"set_validator","","set validator\'s ID",14,[[]]],[11,"set_asset_id","","set assset ID",14,[[["asset",3]]]],[11,"build","","Build block, this method must be called at the very end",14,[[],["block",3]]],[11,"to_string","","Get block info as string",15,[[],["string",3]]],[11,"genesis","","creates genesis block",15,[[],["block",3]]],[11,"is_genesis","","checks if the block is a genesis block",15,[[]]],[11,"hash","","computes the hash of a block, uses the same logic for…",15,[[],["string",3]]],[11,"get_prev","","get hash of previous block",15,[[],[["string",3],["option",4]]]],[11,"get_hash","","get hash of block",15,[[]]],[11,"get_serial_no","","get serial numbr of block",15,[[],["option",4]]],[11,"set_serial_no","","set serial_no of block",15,[[]]],[11,"get_rx","","get receiver involved in the transaction that lead tot the…",15,[[],[["string",3],["option",4]]]],[11,"get_validator","","get validator involved in the creation of this block",15,[[],[["string",3],["option",4]]]],[11,"get_tx","","get sender involved in the transaction that lead tot the…",15,[[],[["string",3],["option",4]]]],[0,"chain","damn_vuln_blockchain","Ledger data-structure for the blockchain",null,null],[3,"Chain","damn_vuln_blockchain::chain","Ledger data-structure for the blockchain",null,null],[3,"AddBlock","","Add Block send block and network_size network_size is…",null,null],[12,"0","","",16,null],[12,"1","","",16,null],[3,"GetLastBlock","","Get last block",null,null],[3,"ReplaceChain","","Replace Chain",null,null],[12,"0","","",17,null],[3,"DumpLedger","","Dumps entire ledger Useful when forking: send `DumpLedger`…",null,null],[11,"new","","create new blockchain",18,[[],["chain",3]]],[11,"get_last_block","","get the last block in the chain",18,[[],["block",3]]],[11,"add_block","","add a block to the chain…",18,[[["block",3]],["chainresult",6]]],[11,"is_valid","","checks if a blockchain is valid by comparing the hash of…",18,[[["vec",3]],["chainresult",6]]],[11,"replace_chain","","",18,[[["vec",3],["block",3]],["chainresult",6]]],[0,"client","damn_vuln_blockchain","Client wrapper for p2p communication",null,null],[3,"Client","damn_vuln_blockchain::client","Client wrapper for p2p communication",null,null],[12,"client","","",19,null],[11,"peer_enroll","","enrolls peer with the auditor enode",19,[[["config",3]]]],[11,"peer_discovery","","gets list of peers from auditor, should be called…",19,[[["config",3]]]],[11,"get_all_assets","","gets asset ledger from auditor node, should be called…",19,[[["config",3]]]],[0,"config","damn_vuln_blockchain","",null,null],[3,"Config","damn_vuln_blockchain::config","",null,null],[12,"mode","","",20,null],[12,"peer_id","","",20,null],[12,"public_ip","","",20,null],[12,"auditor_node","","",20,null],[12,"asset_addr","","",20,null],[12,"chain_addr","","",20,null],[12,"tampered_chain_addr","","",20,null],[12,"network_addr","","",20,null],[12,"init_network_size","","",20,null],[4,"Mode","","",null,null],[13,"Auditor","","",21,null],[13,"Attacker","","",21,null],[13,"Victim","","",21,null],[13,"Normal","","",21,null],[11,"new","","",20,[[]]],[11,"bootstrap","","",20,[[]]],[0,"discovery","damn_vuln_blockchain","Peer management service for synchronising and discovery.…",null,null],[3,"Network","damn_vuln_blockchain::discovery","",null,null],[3,"AddPeer","","Add peer",null,null],[12,"0","","",22,null],[3,"DumpPeer","","Dump all peers",null,null],[3,"GetPeer","","Get peer of ID",null,null],[12,"0","","",23,null],[3,"GetCurrentSize","","Get current network size",null,null],[0,"error","damn_vuln_blockchain","Error datatypes",null,null],[4,"ChainError","damn_vuln_blockchain::error","Errors that can occur when interacting with the blockchain",null,null],[13,"GenesisBlockAdditionError","","when the a genesis block is passed to get added to a chain…",24,null],[13,"InvalidBlockChain","","Blockchain invalid when block.hash() != block.next().prev…",24,null],[13,"InconsistentBlockAdition","","Block inconsistent, block.hash() !=…",24,null],[4,"PeerError","","Errors that can occur when interacting with the blockchain",null,null],[13,"NotAttacker","","When a non-attacking peer is asked to attack",25,null],[13,"NotAuditor","","When a non-auditor peer is asked to mint assets",25,null],[13,"ChainError","","Blockchian error",25,null],[6,"ChainResult","","[Result] datatype for Chain interactions",null,null],[6,"PeerResult","","[Result] datatype for peer interactions",null,null],[0,"logs","damn_vuln_blockchain","",null,null],[3,"Command","damn_vuln_blockchain::logs","",null,null],[12,"timesamp","","",26,null],[12,"action","","",26,null],[4,"Action","","",null,null],[13,"InitLog","","Initialize log",27,null],[13,"PeerConnected","","Peer connected event, pass in peer ID",27,null],[13,"PeerEnroll","","Peer connected event, pass in peer ID",27,null],[13,"MintingAsset","","Miniting asset event, pass in Asset ID",27,null],[13,"DistributingAssets","","Distributing asset `asset ID` to peer `peer  ID`",27,null],[13,"TransactionRequest","","Transaction request event from peer `peer ID` for asset…",27,null],[13,"StakeBroadcast","","Stake broadcast event from peer `peer ID` with stake",27,null],[13,"TransactionValidated","","Transaction validation event. Transaction validated by…",27,null],[13,"BlockCreation","","Block creation event for block `block ID`",27,null],[13,"TransactionBroadcasting","","Transaction broadcasting event by peer `peer ID` to peer…",27,null],[11,"new","","",26,[[["action",4]],["command",3]]],[0,"payload","damn_vuln_blockchain","Payload datatype that `dwb` uses",null,null],[3,"Peer","damn_vuln_blockchain::payload","Represents a peer",null,null],[12,"id","","some random ID",28,null],[12,"ip","","IP must include the port as well",28,null],[3,"Gossip","","",null,null],[12,"tx","","",29,null],[12,"rx","","",29,null],[3,"SellAsset","","Sell asset payload",null,null],[12,"asset_id","","asset ID",30,null],[12,"use_stake","","use stake for transaction?",30,null],[11,"from","damn_vuln_blockchain::asset","",9,[[]]],[11,"into","","",9,[[]]],[11,"to_owned","","",9,[[]]],[11,"clone_into","","",9,[[]]],[11,"to_string","","",9,[[],["string",3]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"vzip","","",9,[[]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_owned","","",10,[[]]],[11,"clone_into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"vzip","","",10,[[]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"to_owned","","",11,[[]]],[11,"clone_into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"vzip","","",11,[[]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"to_owned","","",12,[[]]],[11,"clone_into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"vzip","","",12,[[]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"vzip","","",4,[[]]],[11,"from","","",31,[[]]],[11,"into","","",31,[[]]],[11,"borrow","","",31,[[]]],[11,"borrow_mut","","",31,[[]]],[11,"try_from","","",31,[[],["result",4]]],[11,"try_into","","",31,[[],["result",4]]],[11,"type_id","","",31,[[],["typeid",3]]],[11,"vzip","","",31,[[]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"vzip","","",5,[[]]],[11,"from","","",32,[[]]],[11,"into","","",32,[[]]],[11,"borrow","","",32,[[]]],[11,"borrow_mut","","",32,[[]]],[11,"try_from","","",32,[[],["result",4]]],[11,"try_into","","",32,[[],["result",4]]],[11,"type_id","","",32,[[],["typeid",3]]],[11,"vzip","","",32,[[]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"vzip","","",6,[[]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"to_owned","","",7,[[]]],[11,"clone_into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"vzip","","",7,[[]]],[11,"from","","",13,[[]]],[11,"into","","",13,[[]]],[11,"to_owned","","",13,[[]]],[11,"clone_into","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"vzip","","",13,[[]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"vzip","","",8,[[]]],[11,"from","damn_vuln_blockchain::block","",14,[[]]],[11,"into","","",14,[[]]],[11,"to_owned","","",14,[[]]],[11,"clone_into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"vzip","","",14,[[]]],[11,"from","","",15,[[]]],[11,"into","","",15,[[]]],[11,"to_owned","","",15,[[]]],[11,"clone_into","","",15,[[]]],[11,"to_string","","",15,[[],["string",3]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"vzip","","",15,[[]]],[11,"from","damn_vuln_blockchain::chain","",18,[[]]],[11,"into","","",18,[[]]],[11,"to_owned","","",18,[[]]],[11,"clone_into","","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"vzip","","",18,[[]]],[11,"from","","",16,[[]]],[11,"into","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"vzip","","",16,[[]]],[11,"from","","",33,[[]]],[11,"into","","",33,[[]]],[11,"borrow","","",33,[[]]],[11,"borrow_mut","","",33,[[]]],[11,"try_from","","",33,[[],["result",4]]],[11,"try_into","","",33,[[],["result",4]]],[11,"type_id","","",33,[[],["typeid",3]]],[11,"vzip","","",33,[[]]],[11,"from","","",17,[[]]],[11,"into","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"vzip","","",17,[[]]],[11,"from","","",34,[[]]],[11,"into","","",34,[[]]],[11,"borrow","","",34,[[]]],[11,"borrow_mut","","",34,[[]]],[11,"try_from","","",34,[[],["result",4]]],[11,"try_into","","",34,[[],["result",4]]],[11,"type_id","","",34,[[],["typeid",3]]],[11,"vzip","","",34,[[]]],[11,"from","damn_vuln_blockchain::client","",19,[[]]],[11,"into","","",19,[[]]],[11,"to_owned","","",19,[[]]],[11,"clone_into","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"vzip","","",19,[[]]],[11,"from","damn_vuln_blockchain::config","",20,[[]]],[11,"into","","",20,[[]]],[11,"to_owned","","",20,[[]]],[11,"clone_into","","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"try_into","","",20,[[],["result",4]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"vzip","","",20,[[]]],[11,"from","","",21,[[]]],[11,"into","","",21,[[]]],[11,"to_owned","","",21,[[]]],[11,"clone_into","","",21,[[]]],[11,"borrow","","",21,[[]]],[11,"borrow_mut","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"try_into","","",21,[[],["result",4]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"vzip","","",21,[[]]],[11,"from","damn_vuln_blockchain::discovery","",35,[[]]],[11,"into","","",35,[[]]],[11,"to_owned","","",35,[[]]],[11,"clone_into","","",35,[[]]],[11,"borrow","","",35,[[]]],[11,"borrow_mut","","",35,[[]]],[11,"try_from","","",35,[[],["result",4]]],[11,"try_into","","",35,[[],["result",4]]],[11,"type_id","","",35,[[],["typeid",3]]],[11,"vzip","","",35,[[]]],[11,"from","","",22,[[]]],[11,"into","","",22,[[]]],[11,"borrow","","",22,[[]]],[11,"borrow_mut","","",22,[[]]],[11,"try_from","","",22,[[],["result",4]]],[11,"try_into","","",22,[[],["result",4]]],[11,"type_id","","",22,[[],["typeid",3]]],[11,"vzip","","",22,[[]]],[11,"from","","",36,[[]]],[11,"into","","",36,[[]]],[11,"borrow","","",36,[[]]],[11,"borrow_mut","","",36,[[]]],[11,"try_from","","",36,[[],["result",4]]],[11,"try_into","","",36,[[],["result",4]]],[11,"type_id","","",36,[[],["typeid",3]]],[11,"vzip","","",36,[[]]],[11,"from","","",23,[[]]],[11,"into","","",23,[[]]],[11,"borrow","","",23,[[]]],[11,"borrow_mut","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"try_into","","",23,[[],["result",4]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"vzip","","",23,[[]]],[11,"from","","",37,[[]]],[11,"into","","",37,[[]]],[11,"borrow","","",37,[[]]],[11,"borrow_mut","","",37,[[]]],[11,"try_from","","",37,[[],["result",4]]],[11,"try_into","","",37,[[],["result",4]]],[11,"type_id","","",37,[[],["typeid",3]]],[11,"vzip","","",37,[[]]],[11,"from","damn_vuln_blockchain::error","",24,[[]]],[11,"into","","",24,[[]]],[11,"to_owned","","",24,[[]]],[11,"clone_into","","",24,[[]]],[11,"to_string","","",24,[[],["string",3]]],[11,"borrow","","",24,[[]]],[11,"borrow_mut","","",24,[[]]],[11,"try_from","","",24,[[],["result",4]]],[11,"try_into","","",24,[[],["result",4]]],[11,"type_id","","",24,[[],["typeid",3]]],[11,"vzip","","",24,[[]]],[11,"from","","",25,[[]]],[11,"into","","",25,[[]]],[11,"to_owned","","",25,[[]]],[11,"clone_into","","",25,[[]]],[11,"to_string","","",25,[[],["string",3]]],[11,"borrow","","",25,[[]]],[11,"borrow_mut","","",25,[[]]],[11,"try_from","","",25,[[],["result",4]]],[11,"try_into","","",25,[[],["result",4]]],[11,"type_id","","",25,[[],["typeid",3]]],[11,"vzip","","",25,[[]]],[11,"from","damn_vuln_blockchain::logs","",26,[[]]],[11,"into","","",26,[[]]],[11,"to_string","","",26,[[],["string",3]]],[11,"borrow","","",26,[[]]],[11,"borrow_mut","","",26,[[]]],[11,"try_from","","",26,[[],["result",4]]],[11,"try_into","","",26,[[],["result",4]]],[11,"type_id","","",26,[[],["typeid",3]]],[11,"vzip","","",26,[[]]],[11,"from","","",27,[[]]],[11,"into","","",27,[[]]],[11,"to_string","","",27,[[],["string",3]]],[11,"borrow","","",27,[[]]],[11,"borrow_mut","","",27,[[]]],[11,"try_from","","",27,[[],["result",4]]],[11,"try_into","","",27,[[],["result",4]]],[11,"type_id","","",27,[[],["typeid",3]]],[11,"vzip","","",27,[[]]],[11,"from","damn_vuln_blockchain::payload","",28,[[]]],[11,"into","","",28,[[]]],[11,"to_owned","","",28,[[]]],[11,"clone_into","","",28,[[]]],[11,"to_string","","",28,[[],["string",3]]],[11,"borrow","","",28,[[]]],[11,"borrow_mut","","",28,[[]]],[11,"try_from","","",28,[[],["result",4]]],[11,"try_into","","",28,[[],["result",4]]],[11,"type_id","","",28,[[],["typeid",3]]],[11,"vzip","","",28,[[]]],[11,"from","","",29,[[]]],[11,"into","","",29,[[]]],[11,"to_owned","","",29,[[]]],[11,"clone_into","","",29,[[]]],[11,"to_string","","",29,[[],["string",3]]],[11,"borrow","","",29,[[]]],[11,"borrow_mut","","",29,[[]]],[11,"try_from","","",29,[[],["result",4]]],[11,"try_into","","",29,[[],["result",4]]],[11,"type_id","","",29,[[],["typeid",3]]],[11,"vzip","","",29,[[]]],[11,"from","","",30,[[]]],[11,"into","","",30,[[]]],[11,"borrow","","",30,[[]]],[11,"borrow_mut","","",30,[[]]],[11,"try_from","","",30,[[],["result",4]]],[11,"try_into","","",30,[[],["result",4]]],[11,"type_id","","",30,[[],["typeid",3]]],[11,"vzip","","",30,[[]]],[11,"clone","damn_vuln_blockchain::asset","",9,[[],["asset",3]]],[11,"clone","","",0,[[],["assetledger",3]]],[11,"clone","","",10,[[],["stakebuilder",3]]],[11,"clone","","",1,[[],["stake",3]]],[11,"clone","","",11,[[],["changeassetownerbuilder",3]]],[11,"clone","","",12,[[],["initnetworkbuilder",3]]],[11,"clone","","",13,[[],["setstakebuilder",3]]],[11,"clone","","",7,[[],["setstake",3]]],[11,"clone","damn_vuln_blockchain::block","",14,[[],["blockbuilder",3]]],[11,"clone","","",15,[[],["block",3]]],[11,"clone","damn_vuln_blockchain::chain","",18,[[],["chain",3]]],[11,"clone","damn_vuln_blockchain::client","",19,[[],["client",3]]],[11,"clone","damn_vuln_blockchain::config","",20,[[],["config",3]]],[11,"clone","","",21,[[],["mode",4]]],[11,"clone","damn_vuln_blockchain::discovery","",35,[[],["network",3]]],[11,"clone","damn_vuln_blockchain::error","",24,[[],["chainerror",4]]],[11,"clone","","",25,[[],["peererror",4]]],[11,"clone","damn_vuln_blockchain::payload","",28,[[],["peer",3]]],[11,"clone","","",29,[[],["gossip",3]]],[11,"default","damn_vuln_blockchain::asset","",0,[[],["assetledger",3]]],[11,"default","","",10,[[],["stakebuilder",3]]],[11,"default","","",1,[[],["stake",3]]],[11,"default","","",11,[[],["changeassetownerbuilder",3]]],[11,"default","","",12,[[],["initnetworkbuilder",3]]],[11,"default","","",13,[[],["setstakebuilder",3]]],[11,"default","damn_vuln_blockchain::block","",14,[[],["blockbuilder",3]]],[11,"default","","",15,[[],["block",3]]],[11,"default","damn_vuln_blockchain::client","",19,[[],["client",3]]],[11,"default","damn_vuln_blockchain::discovery","",35,[[],["network",3]]],[11,"default","damn_vuln_blockchain::payload","",28,[[],["peer",3]]],[11,"default","","",29,[[],["gossip",3]]],[11,"eq","damn_vuln_blockchain::asset","",9,[[["asset",3]]]],[11,"ne","","",9,[[["asset",3]]]],[11,"eq","damn_vuln_blockchain::config","",21,[[["mode",4]]]],[11,"eq","damn_vuln_blockchain::error","",24,[[["chainerror",4]]]],[11,"eq","","",25,[[["peererror",4]]]],[11,"ne","","",25,[[["peererror",4]]]],[11,"fmt","damn_vuln_blockchain::asset","",9,[[["formatter",3]],["result",6]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::block","",14,[[["formatter",3]],["result",6]]],[11,"fmt","","",15,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::chain","",18,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::discovery","",35,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::error","",24,[[["formatter",3]],["result",6]]],[11,"fmt","","",25,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::payload","",28,[[["formatter",3]],["result",6]]],[11,"fmt","","",29,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::asset","",9,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::block","",15,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::error","",24,[[["formatter",3]],["result",6]]],[11,"fmt","","",25,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::logs","",26,[[["formatter",3]],["result",6]]],[11,"fmt","","",27,[[["formatter",3]],["result",6]]],[11,"fmt","damn_vuln_blockchain::payload","",28,[[["formatter",3]],["result",6]]],[11,"fmt","","",29,[[["formatter",3]],["result",6]]],[11,"source","damn_vuln_blockchain::error","",25,[[],[["error",8],["option",4]]]],[11,"handle","damn_vuln_blockchain::asset","",0,[[["initnetwork",3]]]],[11,"handle","","",0,[[["dumpledger",3]]]],[11,"handle","","",0,[[["getassetinfo",3]]]],[11,"handle","","",0,[[["changeassetowner",3]]]],[11,"handle","","",0,[[["replaceledger",3]]]],[11,"handle","","",0,[[["choosevalidator",3]]]],[11,"handle","","",0,[[["getpeerassets",3]]]],[11,"handle","","",0,[[["setstake",3]]]],[11,"handle","","",0,[[["getstake",3]]]],[11,"handle","damn_vuln_blockchain::chain","",18,[[["addblock",3]]]],[11,"handle","","",18,[[["getlastblock",3]]]],[11,"handle","","",18,[[["replacechain",3]]]],[11,"handle","","",18,[[["dumpledger",3]]]],[11,"handle","damn_vuln_blockchain::discovery","",35,[[["addpeer",3]]]],[11,"handle","","",35,[[["getpeer",3]]]],[11,"handle","","",35,[[["dumppeer",3]]]],[11,"handle","","",35,[[["getcurrentsize",3]]]],[11,"serialize","damn_vuln_blockchain::asset","",9,[[],["result",4]]],[11,"serialize","","",0,[[],["result",4]]],[11,"serialize","","",1,[[],["result",4]]],[11,"serialize","","",8,[[],["result",4]]],[11,"serialize","damn_vuln_blockchain::block","",14,[[],["result",4]]],[11,"serialize","","",15,[[],["result",4]]],[11,"serialize","damn_vuln_blockchain::chain","",18,[[],["result",4]]],[11,"serialize","damn_vuln_blockchain::discovery","",35,[[],["result",4]]],[11,"serialize","damn_vuln_blockchain::payload","",28,[[],["result",4]]],[11,"serialize","","",29,[[],["result",4]]],[11,"serialize","","",30,[[],["result",4]]],[11,"deserialize","damn_vuln_blockchain::asset","",9,[[],["result",4]]],[11,"deserialize","","",0,[[],["result",4]]],[11,"deserialize","","",1,[[],["result",4]]],[11,"deserialize","","",8,[[],["result",4]]],[11,"deserialize","damn_vuln_blockchain::block","",14,[[],["result",4]]],[11,"deserialize","","",15,[[],["result",4]]],[11,"deserialize","damn_vuln_blockchain::chain","",18,[[],["result",4]]],[11,"deserialize","damn_vuln_blockchain::discovery","",35,[[],["result",4]]],[11,"deserialize","damn_vuln_blockchain::logs","",26,[[],["result",4]]],[11,"deserialize","","",27,[[],["result",4]]],[11,"deserialize","damn_vuln_blockchain::payload","",28,[[],["result",4]]],[11,"deserialize","","",29,[[],["result",4]]],[11,"deserialize","","",30,[[],["result",4]]]],"p":[[3,"AssetLedger"],[3,"Stake"],[3,"ChangeAssetOwner"],[3,"InitNetwork"],[3,"GetAssetInfo"],[3,"ReplaceLedger"],[3,"GetPeerAssets"],[3,"SetStake"],[3,"GetStake"],[3,"Asset"],[3,"StakeBuilder"],[3,"ChangeAssetOwnerBuilder"],[3,"InitNetworkBuilder"],[3,"SetStakeBuilder"],[3,"BlockBuilder"],[3,"Block"],[3,"AddBlock"],[3,"ReplaceChain"],[3,"Chain"],[3,"Client"],[3,"Config"],[4,"Mode"],[3,"AddPeer"],[3,"GetPeer"],[4,"ChainError"],[4,"PeerError"],[3,"Command"],[4,"Action"],[3,"Peer"],[3,"Gossip"],[3,"SellAsset"],[3,"DumpLedger"],[3,"ChooseValidator"],[3,"GetLastBlock"],[3,"DumpLedger"],[3,"Network"],[3,"DumpPeer"],[3,"GetCurrentSize"]]},\
"dwb":{"doc":"Usage:`Damn Vulnerable Blockchain` comes with a peer…","i":[[5,"main","dwb","",null,[[],["result",6]]],[5,"get_json_err","","",null,[[],["jsonconfig",3]]],[0,"routes","","",null,null],[3,"peer_enroll","dwb::routes","",null,null],[3,"peer_dump","","",null,null],[3,"assets_dump","","",null,null],[3,"get_stake","","",null,null],[3,"sell","","",null,null],[5,"services","","",null,[[["serviceconfig",3]]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"vzip","","",4,[[]]],[11,"register","","",0,[[["appservice",3]]]],[11,"register","","",1,[[["appservice",3]]]],[11,"register","","",2,[[["appservice",3]]]],[11,"register","","",3,[[["appservice",3]]]],[11,"register","","",4,[[["appservice",3]]]]],"p":[[3,"peer_enroll"],[3,"peer_dump"],[3,"assets_dump"],[3,"get_stake"],[3,"sell"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);