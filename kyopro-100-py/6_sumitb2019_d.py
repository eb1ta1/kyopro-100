N = int(input())
S = str(input())
cnt = 0
for i in range(10):
    for j in range(10):
        for k in range(10):
            if str(i) in S:
                s_j = S[S.index(str(i))+1:]
                if str(j) in s_j:
                    s_k = s_j[s_j.index(str(j))+1:]
                    if str(k) in s_k:
                        cnt += 1
print(cnt)
