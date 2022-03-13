cargo build --release
# rm ~/bin/git-credential-trdthg
cp ./target/debug/git-credential-trdthg ~/bin
git config --global credential.helper trdthg
git credential-trdthg --file=./creds get

# input the following three rows(include the blank row)
# some info will be printed

# ```
# protocol=https
# host=ggg
#
# ```