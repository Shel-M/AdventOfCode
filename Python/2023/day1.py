sum = 0

def proc(data):
    global sum 
    data = data.strip()
    if len(data) == 0: return 
    min_ = [99, 0] # position, value
    max_ = [-1, 0]

    # this fuckin sucks lol
    for pair in [["one", "1"], 
        ["two", "2"], 
        ["three", "3"], 
        ["four", "4"], 
        ["five", "5"], 
        ["six", "6"], 
        ["seven", "7"], 
        ["eight", "8"], 
        ["nine", "9"], 
        ["zero", "0"]]:
        if pair[0] in data or pair[1] in data: # Pre-check string for existance of either value pairs
            positions = [data.find(pair[0]), data.find(pair[1]), data.rfind(pair[0]), data.rfind(pair[1])] # Find located values in the string, far left and far right
            positions = list(filter(lambda i: i > -1, positions)) # Remove non-matches
            if len(positions) == 0: continue # sanity check, if no matches are left move on

            # Find lowest and highest values, then save it to check against for later loops over this string
            posmin = min(positions)
            posmax = max(positions)
            if posmin < min_[0]: min_ = [posmin, int(pair[1])]
            if posmax > max_[0]: max_ = [posmax, int(pair[1])]

    val = f"{min_[1]}{max_[1]}"
    sum += int(val)


with open("day1.input") as infile:
    for line in infile:
        proc(line)
print(sum)
