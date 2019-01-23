#! /bin/bash -f
for d in */; do
	echo "$d"
	cd $d
	cat src/main.rs | grep "//" | sed 's/^.*\///' | tee notes.md
done
