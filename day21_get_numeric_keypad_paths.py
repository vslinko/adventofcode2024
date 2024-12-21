# 0 = 48
# 1 = 49
# 2 = 50
# 3 = 51
# 4 = 52
# 5 = 53
# 6 = 54
# 7 = 55
# 8 = 56
# 9 = 57
# A = 65

# +---+---+---+
# | 7 | 8 | 9 |
# +---+---+---+
# | 4 | 5 | 6 |
# +---+---+---+
# | 1 | 2 | 3 |
# +---+---+---+
#     | 0 | A |
#     +---+---+
COORDS = {
    48: (1, 3),
    49: (0, 2),
    50: (1, 2),
    51: (2, 2),
    52: (0, 1),
    53: (1, 1),
    54: (2, 1),
    55: (0, 0),
    56: (1, 0),
    57: (2, 0),
    65: (2, 3),
}

all_moves = {}

all_keys = []
max_key = 0

for a in COORDS.keys():
    for b in COORDS.keys():
        k = a-48 + (b-48)*18
        all_keys.append(k)
        max_key = max(max_key, k)

        a_coords = COORDS[a]
        b_coords = COORDS[b]

        dx = b_coords[0] - a_coords[0]
        dy = b_coords[1] - a_coords[1]

        moves = []
        if dx > 0:
            moves.append(">" * dx)
        elif dx < 0:
            moves.append("<" * -dx)

        if dy > 0:
            moves.append("v" * dy)
        elif dy < 0:
            moves.append("^" * -dy)

        moves = "".join(moves)
        moves = list(set([moves, moves[::-1]]))

        if a_coords[0] == 0 and b_coords[1] == 3:
            moves = [m for m in moves if m[0] != "v"]

        if a_coords[1] == 3 and b_coords[0] == 0:
            moves = [m for m in moves if m[0] != "<"]

        moves = list(sorted([m + "A" for m in moves]))

        agg_key = " ".join(["".join(m) for m in moves])
        if agg_key not in all_moves:
            all_moves[agg_key] = {
                "moves": moves,
                "keys": [],
                "comments": [],
            }

        all_moves[agg_key]["keys"].append(str(k))
        all_moves[agg_key]["comments"].append(chr(a) + chr(b))

print('fn get_numeric_keypad_paths(from: u8, to: u8) -> Vec<Vec<u8>> {')
print('    match (from as u16) - 48 + ((to as u16) - 48) * 18 {')
for group in all_moves.values():
    keys = " | ".join(group["keys"])
    moves = ", ".join([
        "vec![{}]".format(", ".join([
            "b'{}'".format(c)
            for c in list(m)
        ]))
        for m in group["moves"]
    ])
    print("        {} => vec![{}],".format(keys, moves))
print("        _ => vec![],")
print("    }")
print("}")
