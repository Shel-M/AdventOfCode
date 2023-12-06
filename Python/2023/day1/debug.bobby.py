tracker = 1
calib = 0
i = 0
with open("day1.input", "r") as mess:
    for line in mess:
        i += 1
        line.replace("\n", "")
        eye =  0
        fst = ""
        lst = ""
        #Set fst to first valid digit
        while fst == "":
            if line[eye].isdigit():
                fst = line[eye]
            elif line[eye:eye + 3] == "one":            
                fst = "1"
            elif line[eye:eye + 3] == "two":
                fst = "2"
            elif line[eye:eye + 5] == "three":
                fst = "3"
            elif line[eye:eye + 4] == "four":
                fst = "4"
            elif line[eye:eye + 4] == "five":
                fst = "5"
            elif line[eye:eye + 3] == "six":
                fst = "6"
            elif line[eye:eye + 5] == "seven":
                fst = "7"
            elif line[eye:eye + 5] == "eight":
                fst = "8"
            elif line[eye:eye + 4] == "nine":
                fst = "9"
            else: eye += 1
        #Reset the eyeball, set lst to last valid digit
        eye = len(line) - 1
        while lst == "":
            if line[eye].isdigit():
                lst = line[eye]
            elif line[eye:eye + 3] == "one":            
                lst = "1"
            elif line[eye:eye + 3] == "two":
                lst = "2"
            elif line[eye:eye + 5] == "three":
                lst = "3"
            elif line[eye:eye + 4] == "four":
                lst = "4"
            elif line[eye:eye + 4] == "five":
                lst = "5"
            elif line[eye:eye + 3] == "six":
                lst = "6"
            elif line[eye:eye + 5] == "seven":
                lst = "7"
            elif line[eye:eye + 5] == "eight":
                lst = "8"
            elif line[eye:eye + 4] == "nine":
                lst = "9"
            else: eye -= 1
        print(i, fst + lst)
        calib += int(fst + lst)
print(calib)
