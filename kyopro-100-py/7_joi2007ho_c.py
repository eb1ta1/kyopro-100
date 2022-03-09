import math

n = int(input())
trees_list = []
max_area = 0

for i in range(n):
    xy = list(map(int, input().split()))
    trees_list.append(xy)

def length(p1,p2):
    p1 = trees_list[p1]
    p2 = trees_list[p2]
    pow = (p1[0]-p2[0])**2 + (p1[1]-p2[1])**2
    return math.sqrt(pow)

for i in range(n):
    for j in range(n):
        for k in range(n):
            for l in range(n):
                if [length(j,k), length(k,l), length(i,l)].count(length(i,j)) == 3 and length(i,k) == length(j,l):
                    print(length(j,k))
                    max_area = max(max_area, length(j,k)**2)

print(int(max_area))
