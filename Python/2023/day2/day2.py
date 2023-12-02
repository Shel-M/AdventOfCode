sum_1 = 0
sum_2 = 0
maximums = {"red": 12, "green": 13, "blue": 14}


def proc(line):
    global sum_1
    global sum_2
    global maximums

    minimums = {"red": 0, "green": 0, "blue": 0}
    line = line.split(":")
    game = line[0][5:] # Save game number
    sets = line[1].split(";") # Break up game sets
    possible = True

    for set in sets: 
        pairs: dict[str, int] = {} 

        vals = set.split(",") # Extract colors
        for val in vals:
            val = val.strip().split() # Break the colors from their values
            pairs.update({str(val[1]): int(val[0])}) # and save in a dictionary
        for k in pairs.keys(): # Go over each color
            # Check if each game is possible by getting the values matching the key in the dictionary
            if maximums.get(k) < pairs.get(k): possible = False 
            # Save the new minimum for a given color if what we're looking at an impossible quantity
            if minimums.get(k) < pairs.get(k): minimums[k] = pairs.get(k)
    if possible:
        sum_1 += int(game)
    # Multiply minimum values together
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

