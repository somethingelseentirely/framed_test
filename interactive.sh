#!/bin/bash
function bot {
   read -n 0 -p $'\033[1;34m'"🤖: $1"$'\033[0m\n'
}

function cmd {
   printf "\033[1;34m🤖: $1\033[0m\n\n"
   printf "\033[0;32m~> \033[0m $2\n"
   eval $2
   read -n 0 -p $'\n\033[1;34m'"🤖: ✅"$'\n\033[0m'
}


bot "Hi I'm 🤖, and I will run you through reproducing this issue today!
Just hit enter whenever you're ready for the next step."
bot "First we're gonna show the current behaviour of FramedRead,
then we're gonna compare that to the default behaviour of AsyncRead."

rm -f test_fifo

cmd "Creating fifo for FramedRead." "mkfifo test_fifo"

bot 'Please run `cargo run -- framed` in a different terminal then come back to me.'

cmd "Writing first message to log." 'echo "this gets read :D" > test_fifo'

cmd "Writing second message to log." 'echo "this gets ignored :[" > test_fifo'

bot "Notice how the second message didn't arrive?
Now let's retry this with the default AsyncRead trait."

bot 'Please stop the running `cargo run` process.'

rm -f test_fifo

cmd "Creating fifo for AsyncRead." "mkfifo test_fifo"

bot 'Please run `cargo run -- async` in a different terminal then come back to me.'

cmd "Writing first message to log." 'echo "this gets read :D" > test_fifo'

cmd "Writing second message to log." 'echo "this gets ignored :[" > test_fifo'

bot "Notice how the second message did in fact arrive this time?"

cmd "Cleaning up by removing fifo." 'rm test_fifo'

bot 'You can now terminate the `cargo run` process,
and head back to the Github issue.
Thanks for running this test!'
