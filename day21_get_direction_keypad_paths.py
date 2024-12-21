# < = 60
# > = 62
# A = 65
# ^ = 94
# v = 118

#     +---+---+
#     | ^ | A |
# +---+---+---+
# | < | v | > |
# +---+---+---+
COORDS = {
    60: (0, 1),
    62: (2, 1),
    65: (2, 0),
    94: (1, 0),
    118: (1, 1),
}

all_moves = {}

for a in COORDS.keys():
    for b in COORDS.keys():
        k = a-60 + (b-60)*3

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

        if a_coords[0] == 0 and b_coords[1] == 0:
            moves = [m for m in moves if m[0] != "^"]

        if b_coords[0] == 0 and a_coords[1] == 0:
            moves = [m for m in moves if m[-1] != "v"]

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


print('fn get_direction_keypad_paths(from: u8, to: u8) -> Vec<Vec<u8>> {')
print('    match from - 60 + (to - 60) * 3 {')
for group in all_moves.values():
    keys = " | ".join(group["keys"])
    moves = ", ".join([
        "vec![{}]".format(", ".join([
            "b'{}'".format(c)
            for c in list(m)
        ]))
        for m in group["moves"]
    ])
    comments = " ".join(group["comments"])
    print("        {} => vec![{}], // {}".format(keys, moves, comments))

print("        _ => vec![],")
print("    }")
print("}")
