import os


CWD = os.path.dirname(os.path.abspath(__file__))
HUMAN_FASTA_FILE = os.path.join(CWD, "..", "data", "chroms", "human.fasta")
HUMAN_CLEANED_TEXT_FILE = os.path.join(CWD, "..", "data", "chroms", "human_cleaned.txt")


def create_reference_text(read_file: str, write_file: str):
    with open(read_file, "r", encoding="utf-8") as f:
        with open(write_file, "w", encoding="utf-8") as f_out:

            while True:
                chunk = f.read(1024 * 1024)
                if not chunk:
                    break

                chunk = chunk.upper()
                data = chunk.split("\n")
                final_data = []
                for line in data:
                    skip = False
                    for c in line:
                        if c not in ["A", "T", "C", "G", "N"]:
                            skip = True
                            break

                    if skip:
                        continue

                    final_data.append(line)
                f_out.write("".join(final_data))



create_reference_text(HUMAN_FASTA_FILE, HUMAN_CLEANED_TEXT_FILE)