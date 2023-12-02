sum_1 = 0
sum_2 = 0
maximums = {"red": 12, "green": 13, "blue": 14}


def proc(line):
    global sum_1
    global sum_2
    global maximums
    minimums = {"red": 0, "green": 0, "blue": 0}

    line = line.split(":")
    game = line[0][5:]
    sets = line[1].split(";")
    possible = True
    for set in sets:
        vals = set.split(",")
        pairs: dict[str, int] = {}
        for val in vals:
            val = val.strip().split()
            pairs.update({str(val[1]): int(val[0])})
        for k in pairs.keys():
            if maximums.get(k) < pairs.get(k): possible = False
            if minimums.get(k) < pairs.get(k): minimums[k] = pairs.get(k)
    if possible:
        sum_1 += int(game)
    print(minimums)
    power =  1
    for n in minimums.values(): power *= n
    sum_2 += power

with open("day2.input") as infile:
    for line in infile:
        line = line.strip()
        proc(line)
print(sum_1)
print(sum_2)
print(maximums)

