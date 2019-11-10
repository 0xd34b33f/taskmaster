#!/usr/bin/env bash
for i in {1..100}; do
  printf "%s$i\n"
  sleep 1
  i+=1
done
