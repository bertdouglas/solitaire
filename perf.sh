# performance tests
# invoke this script from package root

# rebuild in release mode (most optimized)
cargo build -r

# delete and make new logfile for the day
# we don't want to be checking too much into git
# once a day is more than sufficient
logfile="./logs/perf.$(date -I).log"
rm -r $logfile
touch $logfile

# target
tgt="./target/release"

# tests
# invoke directly with highest priority to reduce timing jitter
nice -20 $tgt/test_card_vec_perf | tee -a $logfile
