set -e

launch_verbose() {
	bash -c "exec -a auditor ./target/debug/dwb -m auditor -s 3 -n auditor.batsense.net -i localhost:7000 -a localhost:7000"&
	bash -c "exec -a attacker ./target/debug/dwb -m attacker -s 3 -n attacker.batsense.net -i localhost:7001 -a localhost:7000"&
	bash -c "exec -a victim ./target/debug/dwb -m victim -s 3 -n victim.batsense.net -i localhost:7002 -a localhost:7000"&
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
	kill $(pidof auditor)
	kill $(pidof attacker)
	kill $(pidof victim)
#	kill $(pidof normal)
}

help() {
	cat << EOF
USAGE:
  ./network.sh
  launch   launches network
  kill     kills network
EOF
}

if [ -z $1 ]
then
    help
elif [ $1 == 'launch' ]
then 
	launch_verbose
elif [ $1 == 'kill' ]
then
	kill_network
else
	help
fi
