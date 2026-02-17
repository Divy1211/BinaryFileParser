#  Returns a minimal list of differences between 2 lists e and f
#  requring O(min(len(e),len(f))) space and O(min(len(e),len(f)) * D)
#  worst-case execution time where D is the number of differences.
def lcs(A, B, i = 0, j = 0):
    #  Documented at http://blog.robertelder.org/diff-algorithm/
    N, M, MAX, SIZE = len(A), len(B), len(A) + len(B), len(A) + len(B) + 2 # 2 * min(len(A), len(B)) + 2
    if N > 0 and M > 0:
        delta, vf, vb = N - M, [0] * SIZE, [0] * SIZE
        for D in range(0, (MAX // 2 + (MAX % 2 != 0)) + 1):
            for r in range(0, 2):
                v1, v2, odd, extend = (vf, vb, 1, 1) if r == 0 else (vb, vf, 0, -1)
                for k in range(-(D - 2 * max(0, D - M)), D - 2 * max(0, D - N) + 1, 2):
                    x = v1[(k + 1)] if (k == -D or k != D and v1[(k - 1)] < v1[(k + 1)]) else v1[(k - 1)] + 1
                    y = x - k
                    sx, sy = x, y
                    while x < N and y < M and A[(1 - odd) * N + extend * x + (odd - 1)] == B[(1 - odd) * M + extend * y + (odd - 1)]:
                        x, y = x + 1, y + 1
                    v1[k], z = x, -(k - delta)
                    if MAX % 2 == odd and -(D - odd) <= z <= D - odd and v1[k] + v2[z] >= N:
                        D, x, y, u, v = (2 * D - 1, sx, sy, x, y) if odd == 1 else (2 * D, N - x, M - y, N - sx, M - sy)
                        if D > 1 or (x != u and y != v):
                            return lcs(A[0:x], B[0:y], i, j) + A[x:u] + lcs(A[u:N], B[v:M], i + u, j + v)
                        elif M > N:
                            return A[:N] # diff([], B[N:M], i + N, j + N)
                        else:
                            return B[:M]
                        # elif M < N:
                        #     return diff(A[M:N], [], i + M, j + M)
                        # else:
                        #     return []
    return []
    # elif N > 0:  # Modify the return statements below if you want a different edit script format
    #     return [{"operation": f"delete {A[n]}", "position_old": i + n} for n in range(0, N)]
    # else:
    #     return [{"operation": f"insert {B[n]}", "position_old": i, "position_new": j + n} for n in range(0, M)]


print(lcs([1, 2, 3], [-1, -2, 3]))
