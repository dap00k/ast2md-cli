#!/bin/bash
for filename in ./*.swift; do
    output_filename="$(basename $filename ".swift").ast"
    echo "Producing $output_filename from $filename"
    xcrun swiftc -dump-ast $filename > $output_filename
done
