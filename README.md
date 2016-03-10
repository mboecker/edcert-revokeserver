[![Build Status](https://travis-ci.org/zombiemuffin/edcert-revokeserver.svg?branch=master)](https://travis-ci.org/zombiemuffin/edcert-revokeserver)

# edcert-rejectserver
A TCP based server, which can be used to verify that a certain certificate has not been rejected.

# How it works
A client sends the public key of the certificate in question to this server.
This server checks then, if that public key has been revoked and sends back "ok" or "err".
The check can be done by accessing a database or a file.
