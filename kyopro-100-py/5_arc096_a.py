A, B, C, X, Y = map(int, input().split())
AB_only = A * X + B * Y
C_only = C * max(X, Y) * 2
if X >= Y:
    ABC_price = Y * C * 2 + A * (X - Y)
else:
    ABC_price = X * C * 2 + B * (Y - X)
print(min(AB_only, C_only, ABC_price))
