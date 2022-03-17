# Generate your own self-signed sample identity file

I was able to get a valid pfx file on Windows 10 WSL Ubuntu 20.04 using the following commands:

```
openssl genrsa -out key.pem 512
openssl req -new -key key.pem -out req.csr
openssl x509 -req -in req.csr -signkey key.pem -out cert.crt
openssl pkcs12 -export -out sample_identity.pfx -inkey key.pem -in cert.crt
rm key.pem req.csr cert.crt
```

Adapted from [a post on StackOverflow](https://stackoverflow.com/a/20445432)
