# This script reproduces all save-analysis data in the test_data directories.

# Data for rls-analysis. This is essentially a bootstrap. Be careful when using
# this data because the source is not pinned, therefore the data will change
# regualarly. It should basically just be used as a 'big'-ish set of real-world
# data for smoke testing.
cd ..
RUSTFLAGS=-Zsave-analysis cargo build
rm test_data/rls-analysis/*
cp target/debug/deps/save-analysis/*.json test_data/rls-analysis

# Hello world test case
cd test_data/hello
RUSTFLAGS=-Zsave-analysis cargo build
cp target/debug/save-analysis/hello.json save-analysis
RUSTFLAGS=-Zsave-analysis-api cargo build
cp target/debug/save-analysis/hello.json save-analysis-api
