
./build.sh builds

./run.sh starts a local server for testing. Its uses [miniserve](https://crates.io/crates/miniserve).

View on [http://localhost:8080/browser/index.html](http://localhost:8080/browser/index.html)

## TODO
- Handle callbacks in in main_thread another way. Calls can overwrite the callback of previous calls with their own. Doesn't cause a panic if the same cb function is used every time.
- Handle SQL errors instead of panicking.
- Add support for blobs