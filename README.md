# git-credential-helper

## Introduce

This is a git credential helper which is implemented in rust.
the idea comes from [7.14 Git Tools - Credential Storage](https://git-scm.com/book/en/v2/Git-Tools-Credential-Storage)

## Usage
**Warning!**: this is not a safe credential manager now, don't use it in production

- run `deploy.sh` to test

- put the binary to `PATH`, and add `+x` to it
- set the git with: `git config --global credential.helper trdthg`
- try to use git push

## Feature
- [x] get
- [ ] store
- [ ] erase

- [ ] encryption
- [ ] interact with pass

## About