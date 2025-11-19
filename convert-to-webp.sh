#!/bin/bash

# Manual WebP conversion script
# Usage: ./convert-to-webp.sh <input_image> [quality]

if [ $# -lt 1 ]; then
    echo "Usage: $0 <input_image> [quality]"
    echo "Example: $0 resources/monsters/Textures/Alien_Texture.png 80"
    exit 1
fi

INPUT_FILE="$1"
QUALITY="${2:-80}"  # Default quality 80

if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: File '$INPUT_FILE' not found"
    exit 1
fi

# Get directory and filename without extension
DIR=$(dirname "$INPUT_FILE")
BASENAME=$(basename "$INPUT_FILE" | sed 's/\.[^.]*$//')
OUTPUT_FILE="$DIR/$BASENAME.webp"

echo "Converting: $INPUT_FILE -> $OUTPUT_FILE (quality: $QUALITY)"

# Convert to WebP
cwebp -q "$QUALITY" "$INPUT_FILE" -o "$OUTPUT_FILE"

if [ $? -eq 0 ]; then
    # Show size comparison
    ORIGINAL_SIZE=$(stat -c%s "$INPUT_FILE")
    WEBP_SIZE=$(stat -c%s "$OUTPUT_FILE")
    SAVINGS=$((100 - (WEBP_SIZE * 100 / ORIGINAL_SIZE)))
    
    echo "✅ Conversion successful!"
    echo "   Original: $(numfmt --to=iec-i --suffix=B $ORIGINAL_SIZE)"
    echo "   WebP:     $(numfmt --to=iec-i --suffix=B $WEBP_SIZE)"
    echo "   Savings:  ${SAVINGS}%"
    echo ""
    echo "Next steps:"
    echo "1. Update code references from '$(basename "$INPUT_FILE")' to '$(basename "$OUTPUT_FILE")'"
    echo "2. Test the image loads correctly"
    echo "3. Remove original file: rm '$INPUT_FILE'"
else
    echo "❌ Conversion failed"
    exit 1
fi
