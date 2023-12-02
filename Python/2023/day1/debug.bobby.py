alph = "abcdefghijklmnopqrstuvwxyz"
empty = "                          "
cleanerbook = str.maketrans(alph, empty)
coords = []

i = 0
with open("day1.input", "r") as mess:
    for line in mess:
        i += 1
        line = line.strip()
        if len(line) == 0: continue
        line = line.translate(cleanerbook)
        line = line.replace(" ", "")
        print(i, line)
        fst = line[0]
        lst = line[len(line) - 1]
        print(fst, lst)
        line = fst + lst
        print(line)
        output = int(line)
        print(output)

        coords.append(output)
    print(coords)
    solution = sum(coords)
print(solution) 
