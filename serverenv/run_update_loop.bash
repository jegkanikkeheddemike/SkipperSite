looping=true
#Will keep looping until serverenv exits with an error.
#serverenv will exit with an error if it decides it is time to end the loop
#and shut down
while $looping; do
    #git reset --hard
    #git pull
    /home/$USER/.cargo/bin/cargo run --bin serverenv --release || looping=false
done