with open("./wiki-dump", 'r') as f:
    lines = f.readlines()
print(len(lines))
with open("./dump", 'w') as f:
    for l in lines:
        if len(l) == 0:
            continue
        if l[0] == "\t" or l[0] == " "or l[0] == "\n":
            continue
        if l[:8] == "Contents":
            continue
        l = l.replace("'", "") 
        f.write(l)