from __future__ import print_function
count = 0
for i in range(256):
    if count % 16 == 0:
        print("")
    s = ""
    for b in range(8):
        if ((1 << b) & i) != 0:
            s = "X" + s
        else:
            s = "." + s
    print(":macro " + s + " { :byte " + str(i) + " }", end=' ')
    short = i
    count = count + 1
    shiftcount = 0
    while short & 1 == 0 and shiftcount < 7:
        shiftcount += 1
        short = short >> 1
        s = s[:-1]
        print(":macro " + s + " { :byte " + str(i) + " }", end=' ')
        count = count + 1
