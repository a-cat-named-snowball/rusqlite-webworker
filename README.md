
./build.sh builds

./run.sh starts a local server for testing. Its uses [miniserve](https://crates.io/crates/miniserve).

View on [http://localhost:8080/browser/index.html](http://localhost:8080/browser/index.html)

## TODO
- Get return value back from external call
- Implement execute command
- Create a usable api for main thread to access rusql webworker thread
- Webworker code
- Implement the following api for interacting with the webworker:
```
WebWorker::connect(|conn|{
	conn.execute("SELECT * from data;",|rows|{
		println!("{:}",rows.get(0)?);
	});
});
```