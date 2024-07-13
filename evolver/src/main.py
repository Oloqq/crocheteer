from . import genetic
from deap import tools

ACTIONS_NUM_FOR_GRZIB = 135

def main():
    tb = genetic.init()
    genetic.specimen_initializer(tb, genom_size=ACTIONS_NUM_FOR_GRZIB)
    population = genetic.solve(tb, generations_num=1, population_initializer=3)
    top3 = tools.selBest(population, k=3)
    print(top3[0])