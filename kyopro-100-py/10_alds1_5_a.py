n = int(input())
a = list(map(int,input().split()))
q = int(input())
m = list(map(int,input().split()))
numbers = []

for i in range(2 ** n):
    number = 0
    for j in range(n):
        if (i>>j) & 1:
            number += a[j]
            numbers.append(number)

print(numbers)
"""
for mi in m:
    if mi in numbers:
        print('yes')
    else:
        print('no')
"""