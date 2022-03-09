while True:
    l = list(map(int, input().split()))
    n,x = l[0], l[1]
    if n==0 and x==0:
        break
    else:
        cnt = 0
        for i in range(1,n-1):
            for j in range(i+1, n):
                for k in range(j+1, n+1):
                    if (i+j+k) == x:
                        cnt += 1
        print(cnt)