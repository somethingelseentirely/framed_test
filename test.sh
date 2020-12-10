#!/bin/sh
mkfifo test
echo "this gets read :D" > test_fifo
echo "this gets ignored :[" > test_fifo
