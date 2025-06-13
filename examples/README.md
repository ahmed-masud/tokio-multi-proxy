Run this to generate dev certs:


openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout examples/key.pem -out examples/cert.pem \
  -subj "/CN=localhost"

# Generate CA for mTLS
cp examples/cert.pem examples/ca.pem

