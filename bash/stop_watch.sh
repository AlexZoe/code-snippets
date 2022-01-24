#!/bin/bash

declare -i secs=0
declare -i min=0
declare -i h=0

reset_timer() {
    secs=0
    min=0
    h=0
}

echo ""
while true; do
    printf "\e[1A\e[K%02d:%02d:%02d\n" $h $min $secs
    read -r -s -t 1.0 val; RET=$?
    if [[ $input -eq 0 && $val == "r" ]]; then
        reset_timer
        continue
    fi
    secs+=1
    if [[ $secs == 60 ]]; then
        secs=0
        min+=1
    fi
    if [[ $min == 60 ]]; then
        min=0
        h+=1
    fi
done
