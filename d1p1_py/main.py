path = '../inputs/d1p1.txt'
file = open(path)
sum = 0
for line in file.readlines():
    line_num = ""
    i = 0
    while True:
        if line[i].isdigit():
            line_num += line[i]
            break
        i += 1
    i = len(line) - 1
    while True:
        if line[i].isdigit():
            line_num += line[i]
            break
        i -= 1
    sum += int(line_num);

print(f"sum={sum}")
