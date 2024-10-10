# websink #
An axum playground tool that listens for incoming HTTP and HTTPS connections, printing the URI, headers, and body.

```
$ websink -h
Usage: websink [OPTIONS]

Options:
  -p, --port <PORT>          port number [default: 2024]
  -k, --key <KEY>            key file
  -c, --cert <CERT>          cert file
  -n, --noout                do not produce stdout
  -b, --bytes <BYTES>        body maximum size in bytes [default: 10240]
  -r, --response <RESPONSE>  response file path
  -d, --delay <DELAY>        additional response delay in ms, max 86400000 [default: 0]
  -s, --sink                 sink mode - do nothing, respond with 200, other options ignored
  -h, --help                 Print help
  -V, --version              Print version
```

Response file example response.toml:
```
body = '''
<!DOCTYPE html>
<html>
<body>

<h1>TEST</h1>
<p>Response</p>

</body>
</html>
'''

[headers]
accept-encoding = 'gzip, deflate, br'
sec-fetch-dest = 'image'
Krzysztof = 'Kuś'
```
