from . import genetic, communication
from deap import tools
import json

ACTIONS_NUM_FOR_GRZIB = 133 # 135 - 2 bcs skipping MR and FO

def train():
    tb = genetic.init()
    population = genetic.fresh_population(size=100, genom_size=ACTIONS_NUM_FOR_GRZIB)
    # print(population)
    solutions = genetic.solve(tb, generations_num=1000, population=population)
    top3 = tools.selBest(solutions, k=3)
    print(top3[0])

def get_best():
    path = "population/experiment/generation10.json"
    with open(path) as f:
        population = json.load(f)
    fits = communication.batch_fitness(population)
    fits = [fit[0] for fit in fits]
    m = max(fits)
    ind = fits.index(m)
    best = population[ind]
    print(best)