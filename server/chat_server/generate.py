from OpenSSL.crypto import FILETYPE_ASN1, FILETYPE_PEM, TYPE_RSA, PKey, dump_privatekey, dump_publickey, load_privatekey


keypair = PKey()
keypair.generate_key(TYPE_RSA, 2048)
private_key = keypair.to_cryptography_key()
public_key = private_key.public_key()

v1 = dump_publickey(FILETYPE_PEM, keypair)
v2 = dump_privatekey(FILETYPE_PEM, keypair)

keypair2 = load_privatekey(FILETYPE_PEM, v2)

v3 = dump_publickey(FILETYPE_PEM, keypair2)
v4 = dump_privatekey(FILETYPE_PEM, keypair2)
print(v1)
print(v3)
print(v1 == v3)
