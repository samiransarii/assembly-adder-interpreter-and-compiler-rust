#!/usr/bin/env fish

# Delete old transcript.txt to start fresh
rm -f transcript.txt

# Function to run tests properly
function run_test
    echo "=== Running $argv[1] ===" | tee -a transcript.txt
    cat test/$argv[1].snek | tee -a transcript.txt
    make -B test/$argv[1].run | tee -a transcript.txt
    ./test/$argv[1].run | tee -a transcript.txt
    echo "" | tee -a transcript.txt
end

# Run all test cases
run_test add
run_test sub
run_test negate
run_test mixed
run_test zero
