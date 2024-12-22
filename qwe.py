variants = []

am = 1
mm = 19
bm = am * mm
cm = bm * mm
dm = cm * mm

print(am, bm, cm, dm)

def hash_fn(a, b, c, d):
    return (a+9) + (b+9) * 19 + (c+9) * 361 + (d+9) * 6859

for a in range(-9, 10):
    for b in range(-9, 10):
        for c in range(-9, 10):
            for d in range(-9, 10):
                variants.append(hash_fn(a, b, c, d))

print(len(variants))
print(len(set(variants)))
print(min(variants))
print(max(variants))
