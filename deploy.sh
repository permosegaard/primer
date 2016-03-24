#!/bin/bash

VAGRANT_SSH_OUTPUT="$( vagrant ssh-config )"

function vs_grep {
	formatted="^  ${1} "
	echo "${VAGRANT_SSH_OUTPUT}" | egrep "${formatted}" | sed "s/${formatted}//"
}

function vagrant_ssh_line {
	echo -n "ssh $(
		echo -n "-o UserKnownHostsFile=$( vs_grep "UserKnownHostsFile" ) "
		echo -n "-o StrictHostKeyChecking=$( vs_grep "StrictHostKeyChecking" ) "
		echo -n "-o PasswordAuthentication=$( vs_grep "PasswordAuthentication" ) "
		echo -n "-o IdentitiesOnly=$( vs_grep "IdentitiesOnly" ) "
#		echo -n "-o ProxyCommand=$( vs_grep "ProxyCommand" ) " # quoting hell be here!
		echo -n "-l $( vs_grep "User" ) "
		echo -n "-p $( vs_grep "Port" ) "
		echo -n "-i $( vs_grep "IdentityFile" ) "
		echo -n "$( vs_grep "HostName" )"
	) "
}

if [ "$1" == "profile" ]
then
	cargo build && {
		vagrant rsync && {
			vagrant ssh-config | ssh -F /dev/stdin default "sudo perf record -gq -o /tmp/profile.out /vagrant/target/debug/primer 0 10000000"
			$( vagrant_ssh_line ) -t "sudo perf report -tui -i /tmp/profile.out"
		}
	}
else
	cargo build --release && {
		vagrant rsync && {
			vagrant ssh -- pkill primer # FIXME: catch ctrl-c instead
			vagrant ssh -- /vagrant/target/release/primer
		}
	}
fi
