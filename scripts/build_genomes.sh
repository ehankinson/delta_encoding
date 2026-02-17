#!/usr/bin/env bash
set -e

echo "=== DNA BUILD PIPELINE START ==="

############################################
# Move to repo root (important)
############################################
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$REPO_ROOT"

echo "Running from repo root: $REPO_ROOT"

############################################
# Ensure data/dna folder exists
############################################
mkdir -p data/dna
mkdir -p tmp

############################################
# 1. Download genome if missing
############################################
GENOME_URL="https://hgdownload.soe.ucsc.edu/goldenpath/hg38/bigZips/latest/hg38.chromFa.tar.gz"
TAR_FILE="tmp/hg38.chromFa.tar.gz"

if [ ! -f "$TAR_FILE" ]; then
    echo "Downloading human genome (~1GB)..."
    curl -L "$GENOME_URL" -o "$TAR_FILE"
else
    echo "Genome archive exists — skipping download"
fi

############################################
# 2. Extract chromosome FASTA files
############################################
if [ ! -d "tmp/chroms" ]; then
    echo "Extracting genome..."
    tar -xzf "$TAR_FILE" -C tmp
else
    echo "Chromosome files already extracted"
fi

############################################
# 3. Combine all .fa → human.fasta
############################################
if [ ! -f "data/dna/human.fasta" ]; then
    echo "Combining chromosome FASTA files..."
    cat tmp/chroms/*.fa > data/dna/human.fasta
else
    echo "human.fasta already exists"
fi

############################################
# 4. Clean to single DNA string
############################################
if [ ! -f "data/dna/human_cleaned.txt" ]; then
    echo "Cleaning FASTA -> single DNA string..."
    python3 scripts/dna_cleaner.py
else
    echo "human_cleaned.txt already exists"
fi

############################################
# 5. Generate mutated genomes
############################################
echo "Running mutation generator..."
python3 scripts/mutate_dna.py

echo "=== DONE ==="
