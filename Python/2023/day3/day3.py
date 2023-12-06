import re
def checkline(inline, pos):
    line = inline
    start = 0
    end = len(line)
    if pos[0] > 0:
        start = pos[0] - 1
    if pos[0] + pos[1] + 1 < len(line):
        end = pos[0] + pos[1] + 1

    line = line[start:end]
    findline = re.sub(r"[\.\d]", "", line)
    if len(findline):
        cogpos = inline.find(findline, start, end)
        if findline != "*": cogpos = -1
        return [True, cogpos]
    return [False, -1]

linearr = []
with open("day3.input") as infile:
    for line in infile:
        linearr.append(line.strip())

s = 0
s2 = []
cogs = {}
for lineidx, line in enumerate(linearr):
    numbers = re.split(r"\D", line)
    numbers = list(map(lambda n: re.sub(r"\D", "", n), numbers))
    numbers = list(filter(lambda n: len(n) > 0, numbers))
    positions = [[0,0]]
    for n in numbers:
        f = line.find(n, positions[len(positions)-1][0]+positions[len(positions)-1][1])
        if f > -1:
            positions.append([f, len(n)])
    positions = positions[1:]
    parts = []
    for posidx, pos in enumerate(positions):
        for l in range(-1,2):
            offsetlineidx = lineidx + l
            try:
                chkline = checkline(linearr[offsetlineidx], pos)
                if chkline[0]:
                    parts.append(int(numbers[posidx]))
                    if chkline[1] > -1:
                        key = f'{offsetlineidx}x{chkline[1]}'
                        v = {key: int(numbers[posidx])}
                        print(v)
                        print(cogs)
                        if key not in cogs.keys():
                            cogs.update(v)
                            print("appending cog")
                        else:
                            s2.append(v[key] * cogs[key])
                            cogs.pop(key)
                            print("removing cog")
                        print(cogs)
            except: continue
    print(lineidx+1, parts)
    print()
    s += sum(parts) 
print(s)
print(sum(s2))

