# RSA Pam
Pam through RSA.

## Generating keys
### Private
`openssl genrsa -out <private_key_file> 1024`

### Public
`openssl rsa -in <private_key_file> -out <public_key_file> -pubout -outform PEM`