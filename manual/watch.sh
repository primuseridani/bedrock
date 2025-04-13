#!/usr/bin/env sh

f="bedrock.tex"

while :
do
	while inotifywait -e close_write "${f}"
	do
			make
	done
done
