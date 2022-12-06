def main():
    max_load: int = 0
    with open("input.txt", "rt") as f:
        this_load: int = 0
        while line := f.readline():
            # blank line = new elf: add to the list
            line = line.strip()
            if not line:
                max_load = max(max_load, this_load)
                this_load: int = 0
            else:
                this_load += int(line)

    max_load = max(max_load, this_load)
    print(max_load)

if __name__ == "__main__":
    exit(main())

