# actions_for_grzib = 135

from . import genetic
from deap import tools

def main():
    tb = genetic.init_toolbox()
    population = genetic.solve(tb)
    top10 = tools.selBest(population, k=10)
    print(top10[0])