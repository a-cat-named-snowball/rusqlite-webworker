
./build.sh builds

./run.sh starts a local server for testing. Its uses [miniserve](https://crates.io/crates/miniserve).

View on [http://localhost:8080/browser/index.html](http://localhost:8080/browser/index.html)

## TODO

- Return queried rows from web worker instead of dummy data
- Fix recursive mutex error in main_thread callbacks
- Handle SQL errors instead of panicking