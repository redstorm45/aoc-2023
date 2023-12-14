#!/bin/bash

PROGCOUNT=14
outfile=$(mktemp)


echo "Build all..."
for i in {1..14}
do
	dirname="aoc$(printf "%02d" $i)"
	(cd $dirname && cargo build)
done


echo "\nTiming programs..."
for i in {1..14}
do
	p=$(printf "%02d" $i)
	dirname="aoc${p}"
	echo "Running [$p]"
	\time --format="[$p] %E" -o $outfile -a "./${dirname}/target/debug/${dirname}" "./${dirname}/input"
done

echo "\nTimes:"
cat $outfile

rm $outfile
