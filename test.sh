#!/usr/bin/bash
# A simple script that generates random numbers

if [[ $# == 0 ]] ; then
	TOTAL=4096
else
	TOTAL="$1"
fi

for i in $(seq 2 $TOTAL) ; do
	if (( $RANDOM % 2 )) ; then
		echo "-$RANDOM.$RANDOM"
	else
		echo "$RANDOM.$RANDOM"
	fi
done

exit
