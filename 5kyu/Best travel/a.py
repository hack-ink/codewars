import itertools
def choose_best_sum(t, k, ls):
    try: 
        return max(sum(i) for i in itertools.combinations(ls,k) if sum(i)<=t)
    except:
        return None

xs = [253, 244, 167, 97, 348, 249, 296, 321, 81, 433]
print choose_best_sum(1437, 5, xs)
