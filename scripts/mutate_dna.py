import os
import random
from multiprocessing import Pool

CWD = os.path.dirname(os.path.abspath(__file__))
REFERENCE_FILE = os.path.join(CWD, "..", "data", "dna", "human_cleaned.txt")
GENOME_FILES = os.path.join(CWD, "..", "data", "dna", "human_mutated_genome_{i}.txt")

NUM_GENOMES = 5
MUTATION_RATE = 0.001   # 0.1%
CHUNK_SIZE = 1024 * 1024  # 1MB

bases = ["A", "C", "G", "T"]

def mutate_char(c):
    if c not in "ACGT":
        return c

    if random.random() < MUTATION_RATE:
        return random.choice(bases)
    return c

def generate_genome(i):
    random.seed(69 * 420 * 67 + i)
    output_file = GENOME_FILES.format(i=i)
    print(f"Creating {output_file}...")

    with open(REFERENCE_FILE, "r") as ref:
        with open(output_file, "w") as out:
            while True:
                chunk = ref.read(CHUNK_SIZE)
                if not chunk:
                    break

                mutated = "".join(mutate_char(c) for c in chunk)
                out.write(mutated)

    print(f"genome_{i}.txt complete")

if __name__ == "__main__":
    with Pool(processes=NUM_GENOMES) as pool:
        pool.map(generate_genome, range(1, NUM_GENOMES + 1))

    print("All genomes generated.")