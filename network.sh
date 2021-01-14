set -e

launch_debug() {
	bash -c "exec -a auditor ./target/debug/dwb -m auditor -s 3 -n auditor.batsense.net -i localhost:7000 -a localhost:7000"&
	bash -c "exec -a attacker ./target/debug/dwb -m attacker -s 3 -n attacker.batsense.net -i localhost:7001 -a localhost:7000"&
	bash -c "exec -a victim ./target/debug/dwb -m victim -s 3 -n victim.batsense.net -i localhost:7002 -a localhost:7000"&
	#bash -c "exec -a normal ./target/debug/dwb -m normal -s 3 -n normal.batsense.net -i localhost:7003 -a localhost:7000"&
}

launch_release() {
	bash -c "exec -a auditor ./target/release/dwb -m auditor -s 3 -n auditor.batsense.net -i localhost:7000 -a localhost:7000"&
	bash -c "exec -a attacker ./target/release/dwb -m attacker -s 3 -n attacker.batsense.net -i localhost:7001 -a localhost:7000"&
	bash -c "exec -a victim ./target/release/dwb -m victim -s 3 -n victim.batsense.net -i localhost:7002 -a localhost:7000"&
	#bash -c "exec -a normal ./target/debug/dwb -m normal -s 3 -n normal.batsense.net -i localhost:7003 -a localhost:7000"&
}


#launch() {
#	for i in [ "victim" "normal" "attacker" "auditor" ]
#	do
#	bash -c "exec -a $i ./target/debug/dwb -m $i -s 3 -n $i.batsense.net -i localhost:5003 -a localhost:5000"&
#
#	#cho " ./target/debug/dwb -m $i -s 3 -n $i.batsense.net -i localhost:5003 -a localhost:5000"&
#	done
#}


kill_network(){
	kill -9 $(pidof auditor)
	kill -9 $(pidof attacker)
	kill -9 $(pidof victim)
#	kill $(pidof normal)
}

help() {
	cat << EOF
USAGE:
  ./network.sh
  launch		 launches network
		release  launches network with release binary
  kill			 kills network
EOF
}

if [ -z $1 ]
then
    help
elif [ $1 == 'launch' ]
then 
	if [ -z $2 ]
	then
		launch_debug
	elif [ $2 == 'release' ]
	then
		launch_release
	else 
		help
	fi
elif [ $1 == 'kill' ]
then
	kill_network
else
	help
fi
