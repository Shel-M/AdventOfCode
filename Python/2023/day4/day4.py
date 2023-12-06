sum = 0
cards = {}
with open("day4.input") as infile:
    # record all cards to dictionary
    for line in infile:
        cardnum = {line.split(":")[0].split()[1]: 1}
        cards.update(cardnum)

    infile.seek(0) # reset file read
    for line in infile:
        cardnum = int(line.split(":")[0].split()[1])
        card = line.split(":")[1].split("|")
        winning = card[0].split()
        current = card[1].split()

        count = 0
        for number in current:
            if number in winning:
                count += 1
        if count > 0:
            for i in range(1, count+1): 
                cards[str(i + cardnum)] += 1 * cards[str(cardnum)]
for card in cards.values():
    sum += card
print(sum)
