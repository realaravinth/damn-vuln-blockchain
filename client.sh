#!/bin/bash


# usage get_all_assets <peer IP>
get_all_assets() {
	curl --location  \
		--request GET "$1/assets/all" \ | prettify
}

prettify() {
	jq --color-output | less -R
}

# usage set_attack <peer IP>
set_attack() {
	curl --location \
		--request POST "$1/attack"
}

# usage sell_asset <peer IP> <asset ID> <buyer peer id>
sell_asset() {
# echo "$(make_sell_payload $2 $3)"
 	 curl --location --request POST "$1/assets/sell" --header 'Content-Type: application/json' --data-raw $(make_sell_payload $2 $3)
}


# Get all peers in network
# usage get_peers <peer IP>
get_peers() {
	curl --location \
		--request GET 'localhost:7000/peers/all' | prettify 
}

# usage make_sell_payload <asset ID> <buyer peer id>
make_sell_payload() {
	echo "'{ \"asset_id\": \"$1\", \"buyer_peer_id\": \"$2\"}'"
}
