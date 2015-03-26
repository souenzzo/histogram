#!/usr/bin/bash
# A simple script that generates random numbers

if [[ $# == 0 ]] ; then
	TOTAL=4096
else
	TOTAL="$1"
fi

for i in $(seq 1 $TOTAL) ; do
	echo $RANDOM
done

exit
