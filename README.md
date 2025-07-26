# About this library
This library builds on top of the `std::fs` library to allow a niche edge case where a user wants to readlink to a file without requiring it to exist. The problem with `std::fs::canonicalize` is that it needs the readlinked file to exist or it will just return an err variant (not resolving the link). 


# Testing Locally
Run `scripts/generate_symlinks.sh` before running `cargo test`

# Testing in Docker
Run `docker compose up`
