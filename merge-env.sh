OUTPUT=".env"
SRC_FILES=("rust-test/.env" "svelte-test/.env")

echo "# Auto-generated env file" > $OUTPUT
echo "# Do not edit manually" >> $OUTPUT

for file in "${SRC_FILES[@]}"; do
  if [ -f "$file" ]; then
    echo "" >> $OUTPUT
    echo "# ----------------------------------------" >> $OUTPUT
    echo "" >> $OUTPUT
    echo "### $file ###" >> $OUTPUT
    echo "" >> $OUTPUT
    cat "$file" >> $OUTPUT
    echo "" >> $OUTPUT
  fi
done

echo "✅ Merged env into $OUTPUT"