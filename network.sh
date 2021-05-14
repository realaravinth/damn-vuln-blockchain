set -e

launch_auditor() {
	bash -c "exec -a auditor ./target/release/dwb -m auditor -s 3 -n auditor.batsense.net -i 0.0.0.0:7000 -a localhost:7000"&
}

launch_normal() {
	bash -c "exec -a normal ./target/debug/dwb -m normal -s 3 -n normal.batsense.net -i localhost:7003 -a localhost:7000"&
}

launch_attacker() {
	bash -c "exec -a attacker ./target/debug/dwb -m attacker -s 3 -n attacker.batsense.net -i localhost:7001 -a localhost:7000"&
}

launch_victim() {
	bash -c "exec -a victim ./target/debug/dwb -m victim -s 3 -n victim.batsense.net -i localhost:7002 -a localhost:7000"&
}

launch_release() {
	bash -c "exec -a attacker ./target/release/dwb -m attacker -s 3 -n attacker.batsense.net -i localhost:7001 -a localhost:7000"&
	bash -c "exec -a victim ./target/release/dwb -m victim -s 3 -n victim.batsense.net -i localhost:7002 -a localhost:7000"&
	bash -c "exec -a normal ./target/release/dwb -m normal -s 3 -n normal.batsense.net -i localhost:7003 -a localhost:7000"&
}

kill_auditor() {
	kill -9 $(pidof auditor)
}

kill_normal() {
	kill $(pidof normal)
}

kill_victim() {
kill -9 $(pidof victim)
}

kill_attacker() {
	kill -9 $(pidof attacker)
}

kill_test_net(){
	kill_victim
	kill_attacker
	kill_auditor
}

launch_test_net() {
	launch_auditor
	launch_attacker
	launch_victim
}

launch_production() {
	launch_attacker
	launch_victim
	launch_normal
}

kill_production() {
	kill_victim
	kill_attacker
	kill_normal
}

help() {
	cat << EOF
USAGE:
  ./network.sh
  launch		 launches test network
		release  launches network in production setup(seperate auditor launch)
		auditor  launches auditor
  kill			 kills test network
		release  kills network in production setup(seperate auditor launch)
		auditor  kills auditor
EOF
}

if [ -z $1 ]
then
    help
elif [ $1 == 'launch' ]
then 
	if [ -z $2 ]
	then
		launch_test_net
	elif [ $2 == 'release' ]
	then
		launch_release
	elif [ $2 == 'auditor' ]
	then
		launch_auditor
	else 
		help
	fi
elif [ $1 == 'kill' ]
then
	if [ -z $2 ]
	then
		kill_test_net
	elif [ $2 == "release" ]
	then
		kill_production
	elif [ $2 == "auditor" ]
	then
		kill_auditor
	else
		help
	fi
else
	help
fi
