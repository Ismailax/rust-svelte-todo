#!/usr/bin/env sh
set -eu

# สร้างไฟล์ให้ permission แน่นเสมอ
umask 077

write_pem () {
  var_name="$1"
  out_path="$2"

  val="$(printenv "$var_name" 2>/dev/null || true)"
  if [ -n "$val" ]; then
    printf "%s" "$val" > "$out_path"
    return 0
  fi

  val_b64="$(printenv "${var_name}_B64" 2>/dev/null || true)"
  if [ -n "$val_b64" ]; then
    printf "%s" "$val_b64" | base64 -d > "$out_path"
    return 0
  fi

  return 1
}

# เขียน private key
if write_pem "JWT_PRIVATE_KEY_PEM" "/tmp/private_key.pem"; then
  export JWT_PRIVATE_KEY_PATH="/tmp/private_key.pem"
else
  echo "Missing JWT_PRIVATE_KEY_PEM (or JWT_PRIVATE_KEY_PEM_B64)" >&2
  exit 1
fi

# เขียน public key
if write_pem "JWT_PUBLIC_KEY_PEM" "/tmp/public_key.pem"; then
  export JWT_PUBLIC_KEY_PATH="/tmp/public_key.pem"
else
  echo "Missing JWT_PUBLIC_KEY_PEM (or JWT_PUBLIC_KEY_PEM_B64)" >&2
  exit 1
fi

exec "$@"
SH

chmod +x rust-test/entrypoint.sh