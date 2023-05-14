# performance tests
logfile="./logs/perf.$(date -I).log"
rm -r $logfile
touch $logfile
cargo run -r --bin test_card_vec_perf | tee >> $logfile
