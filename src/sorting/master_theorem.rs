/*
Given T(n) = aT(n/b) + f(n), where a >= 1, b > 1, 
compare f(n) with n^(log_b(a)) to determine 
the asymptotic behavior of T(n):

case 1: f(n) = O(n^(log_b(a) - ε)) for some ε > 0
    T(n) = Θ(n^(log_b(a)))

case 2: f(n) = Θ(n^(log_b(a))
both equal : T(n) = Θ(n^(log_b(a)) * log(n))

case 3: f(n) = Ω(n^(log_b(a) + ε)) for some ε > 0

work outside recursive calls dominates: T(n) = Θ(f(n))

*/