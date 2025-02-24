#!/bin/bash

# List of features (benchmarks) to iterate over
FEATURES=(
    "hyped"
    "maud"
    "askama"
    "tera"
    "tinytemplate"
    "handlebars"
    "minijinja"
    "html_node"
    "hypertext"
    "vy"
    "fhtml"
)

# Iterate over each feature
for FEATURE_NAME in "${FEATURES[@]}"; do
    echo "Running benchmarks for feature: $FEATURE_NAME"

    # Clean the project
    echo "Running cargo clean..."
    cargo clean

    # Run benchmarks with the current feature
    echo "Running cargo bench for $FEATURE_NAME..."
    cargo bench -Z unstable-options --timings=html --features "$FEATURE_NAME"
    LINK='<hr><a href="cargo-timing-'
    LINK2='.html">'
    cat "./target/cargo-timings/cargo-timing.html" > "./docs/cargo-timing-$FEATURE_NAME.html"
    echo "${LINK}${FEATURE_NAME}${LINK2}${FEATURE_NAME}</a>" >> "docs/list.html"

    # Check if the command succeeded
    if [ $? -eq 0 ]; then
        echo "Benchmarks for $FEATURE_NAME completed successfully."
    else
        echo "Benchmarks for $FEATURE_NAME failed."
        exit 1
    fi

    echo "----------------------------------------"
done

echo "All benchmarks completed."
