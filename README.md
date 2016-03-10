[![Build Status](https://travis-ci.org/zombiemuffin/edcert-revokeserver.svg?branch=master)](https://travis-ci.org/zombiemuffin/edcert-revokeserver)

# edcert-revokeserver
A TCP based server, which can be used to verify that a certain certificate has not been revoked.

# How it works
A client sends the public key of the certificate in question to this server.
This server checks then, if that public key has been revoked and sends back "ok" or "err".
The check can be done by accessing a database or a file.

Because of the very simple design of this service, it should __not be able to DDoS it__,
and it __scales very simple__ with more servers.

# Todo (in no order)
- Encryption (maybe by using HTTPS?)
- Authentication
- Query, if certificate has been revoked
- Automatically revoke certificates using the private key
