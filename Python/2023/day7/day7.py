from collections import Counter

card_key = {"A": 13, "K": 12, "Q": 11, "J": 10, "T": 9, "9": 8, "8": 7, "7": 6, "6": 5, "5": 4, "4": 3, "3": 2, "2": 1}
card_key_p2 = {"A": 12, "K": 11, "Q": 10, "T": 9, "9": 8, "8": 7, "7": 6, "6": 5, "5": 4, "4": 3, "3": 2, "2": 1, "J": 0}
match_types = {"FiveKind": 6, "FourKind": 5, "FullHouse": 4, "ThreeKind": 3, "TwoPair": 2, "Pair": 1, "HighCard": 0}

class Hand:
    hand = ""
    bid = 0
    match_type = "HighCard"
    value = []

    def __str__(self) -> str: return f"({self.hand}, {self.value})"
    def __repr__(self) -> str: return self.__str__() + "\n" 

    def __init__(self, line):
        line = line.split()
        self.hand = line[0]
        self.bid = int(line[1])
        self.set_type()

    def set_type(self, part=1):
        vals = []
        c = dict(Counter(self.hand))
        j = 0
        if part == 2:
            c = dict(Counter(self.hand))
            print(c)
            if c.get("J"): j = c.pop("J")

            print(j, c)

        vals = sorted(c.values())
        vals.reverse()
        if len(vals) == 0: vals = [5]
        else: vals[0] += j

        if vals[0] == 1: self.match_type = "HighCard"
        elif vals[0] == 5: self.match_type = "FiveKind"
        elif vals[0] == 4: self.match_type = "FourKind"
        elif vals[0:2] == [3, 2]: self.match_type = "FullHouse"
        elif vals[0] == 3: self.match_type = "ThreeKind"
        elif vals[0] == 2 and vals[1] == 2: self.match_type = "TwoPair"
        elif vals[0] == 2: self.match_type = "Pair"
        elif vals[0] == 1: self.match_type = "HighCard"
        else: raise(ValueError(f"{hand} ({vals})"))
        self.set_value(part)
    def set_value(self, part=1): 
        v = [match_types.get(self.match_type)]
        ck = card_key
        if part == 2: ck = card_key_p2
        for c in self.hand: v.append(ck.get(c))
        self.value = tuple(v)

file = open("input")

hands = [Hand(file.readline())]
for line in file.readlines():
    hand = Hand(line)
    hands.append(hand)

hands.sort(key=lambda x: x.value )
#hands.reverse()
print(len(hands))
s = 0
for i, h in enumerate(hands): 
    s += (i+1) * h.bid
    hands[i].set_type(part=2)
print("Part 1:", s)

hands.sort(key=lambda x: x.value )
s = 0
for i, h in enumerate(hands): 
    s += (i+1) * h.bid
print("Part 2:", s)
