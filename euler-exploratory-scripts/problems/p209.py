from itertools import product
from random import choice

def action(point):
    a, b, c, d, e, f = point
    return b, c, d, e, f, a ^ (b & c)

def generate_random_point():
    res = []
    for _ in range(6):
        res.append(choice([0, 1]))
    return tuple(res)

def generate_random_orbit():
    seen = set()
    orbit = []
    start = generate_random_point()
    while start not in seen:
        orbit.append(start)
        seen.add(start)
        start = action(start)
    return orbit

def generate_orbit(point):
    seen = set()
    orbit = []
    start = point
    while start not in seen:
        orbit.append(start)
        seen.add(start)
        start = action(start)
    return orbit

def orbit_decomposition():
    seen = set()
    orbits = []
    for point in product(*[[0,1] for _ in range(6)]):
        if point not in seen:
            orbit = generate_orbit(point)
            orbits.append(orbit)
            seen = seen.union(set(orbit))
    return orbits
