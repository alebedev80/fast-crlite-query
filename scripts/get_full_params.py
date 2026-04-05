import sys
import struct
from cryptography import x509
from cryptography.hazmat.primitives import hashes, serialization

def get_params(cert_path, issuer_path):
    with open(cert_path, 'rb') as f:
        leaf = x509.load_pem_x509_certificate(f.read())
    with open(issuer_path, 'rb') as f:
        issuer = x509.load_pem_x509_certificate(f.read())

    serial = hex(leaf.serial_number)[2:].lower().rstrip('l')
    if len(serial) % 2 != 0: serial = "0" + serial

    issuer_spki = issuer.public_key().public_bytes(
        encoding=serialization.Encoding.DER, format=serialization.PublicFormat.SubjectPublicKeyInfo
    )
    h = hashes.Hash(hashes.SHA256()); h.update(issuer_spki)
    issuer_hash = h.finalize().hex()

    sct_list = []
    try:
        ext = leaf.extensions.get_extension_for_oid(x509.oid.ExtensionOID.PRECERT_SIGNED_CERTIFICATE_TIMESTAMPS)
        for sct in ext.value:
            log_id = sct.log_id.hex()
            ts_ms = int(sct.timestamp.timestamp() * 1000)
            sct_list.append(f"{log_id}:{ts_ms}")
    except: pass

    print(f"{issuer_hash} {serial} {' '.join(sct_list)}")

if __name__ == "__main__":
    get_params(sys.argv[1], sys.argv[2])
