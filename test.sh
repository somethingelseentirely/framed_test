#!/bin/sh
mkfifo test_fifo
echo "this gets read :D" > test_fifo
echo "this gets ignored :[" > test_fifo
