## IAM Token Manager
The IAM Token Manager manages IAM access tokens and refreshes them before they
expire. Currently only IBM IAM tokens are supported.

![demo](.github/iam.svg)

### Prerequisites
Install the current stable release of Rust by using rustup. On Linux & OS X
this can be done by running:

```
curl https://sh.rustup.rs -sSf | sh
```



### IBM
Compile the `ibmtest` example binary by executing the following in the repo root:

```
cargo build --example ibmtest 
```

The compiled binary `ibmtest` can be found in `target/debug/examples/`.

The binary has the following command line flags 

- `--ibm`: pass in an API key for `cloud.ibm.com` (can be passed multiple times)
- `--ibm-test`: pass in an API key for `test.cloud.ibm.com` (can be passed multiple times)
- `--web.listen-port`: port for web server (optional)
- `--token-refresh-seconds`: how frequently a new access token for each API key is requested (optional, default=1800)


At least one of `--ibm` or `--ibm-test` needs to be specified. If `web.listen-port` is 
specified, a web server is started that exposes the following routes:

- GET `/v1/access_tokens`: list of all current access tokens mapped by BSS account. Sample response:

```
{
  "access_tokens": [
    {
      "<BSS account id>": "<access token>"
    }
  ]
}
```

- GET `/v1/access_token/<id>`: access token for account with ID `<id>`. Sample response:

```
{
  "access_token": "<access token>"
}
```

#### Example (replace with real API keys):

The environment variable `RUST_LOG` controls the logging output detail. Not setting this environment
variable means that only errors will be logged:

```
RUST_LOG=info ./target/debug/examples/ibmtest --ibm <key1> --ibm <key2> --ibm-test <key3> --web.listen.port=5050
```

