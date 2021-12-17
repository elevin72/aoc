
from typing import Set, List
import math


def dijk(graph: List[List[int]]):
    length = len(graph)
    Q = set()
    dist = [[math.inf for _ in range(length)] for _ in range(length)]
    prev = [[-1 for _ in range(length)] for _ in range(length)]

    for (i, row) in enumerate(graph):
        for j in row:
            Q.add((i,j))

    dist[0][0] = 0
    
    def k(a):
        y = a[0]
        x = a[1]
        return dist[y][x]

    while len(Q) != 0:
        u = min(Q, key=k)

        Q.remove(u)

        for (dy, dx) in [(0,-1),(1,0),(0,1),(-1,0)]:
            r = u[0] + dy
            c = u[1] + dx
            if r >= 0 and r < length and c >= 0 and c < length:
                alt = dist[u[0]][u[1]] + graph[r][c]
                if alt < dist[r][c]:
                    dist[r][c] = alt
                    prev[r][c] = u

    return dist, prev
            




graph = [
    [1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
    [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
    [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
    [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
    [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
    [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
    [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
    [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
    [1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
    [2, 3, 1, 1, 9, 4, 4, 5, 8, 1]]

print(dijk(graph))
