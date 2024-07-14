from . import genetic
from deap import tools

ACTIONS_NUM_FOR_GRZIB = 133 # 135 - 2 bcs skipping MR and FO

def main():
    tb = genetic.init()
    population = genetic.fresh_population(size=100, genom_size=ACTIONS_NUM_FOR_GRZIB)
    # print(population)
    solutions = genetic.solve(tb, generations_num=1000, population=population)
    top3 = tools.selBest(solutions, k=3)
    print(top3[0])