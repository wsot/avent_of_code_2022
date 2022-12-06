def main():
    elfloads: list[int] = []
    with open("input.txt", "rt") as f:
        elfload: int = 0
        while line := f.readline():
            # blank line = new elf: add to the list
            line = line.strip()
            if not line:
                elfloads.append(elfload)
                elfload = 0
                continue
            elfload += int(line)

        elfloads.append(elfload)

    print(sum(sorted(elfloads, reverse=True)[:3]))

if __name__ == "__main__":
    exit(main())

