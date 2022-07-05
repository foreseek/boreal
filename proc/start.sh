#!/bin/bash
if [ ! -f "./app/assets/" ] 
then
  mkdir ./app/assets/
fi

cargo watch -w server -w src -x 'run' &
cd app && trunk watch