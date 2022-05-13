
./build.sh builds

./run.sh starts a local server for testing. Its uses [miniserve](https://crates.io/crates/miniserve).

View on [http://localhost:8080/browser/index.html](http://localhost:8080/browser/index.html)

## TODO

- Fully implement execute command
- Fully implement query command
- Improve api for main thread to access rusql webworker thread
- Have webworker thready apply sql commands that it recieves and return real output
- Better data format for returning rows
