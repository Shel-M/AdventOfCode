file = open("day6.input")

times = list(map(lambda x: int(x), file.readline().split(":")[1].split()))
distances = list(map(lambda x: int(x), file.readline().split(":")[1].split()))


print(times)
print(distances)
res = []
for pair in range(0, len(times)):
    ways = 0 
    for x in range(0, times[pair]+1):
        y = ((-1)*(x**2)) + (times[pair] * x)
        if y > distances[pair]: ways += 1
            
    print(ways)
    res.append(ways)

s = 1
for r in res:
    s *= r
print("part 1:", s)

file.seek(0)
time = int(file.readline().split(":")[1].replace(" ", ""))
dist = int(file.readline().split(":")[1].replace(" ", ""))

print(time, dist)
ways = 0
for x in range(0, time+1):
        y = ((-1)*(x**2)) + (time * x)
        if y > dist: ways += 1
print("part 2:", ways)

