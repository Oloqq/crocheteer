from .communication import fitness, batch_fitness
import random
from deap import creator, base, tools, algorithms

def init():
    creator.create("FitnessMax", base.Fitness, weights=(1.0,))
    creator.create("Individual", list, fitness=creator.FitnessMax)

    toolbox = base.Toolbox()

    toolbox.register("evaluate", fitness)
    toolbox.register("mate", tools.cxTwoPoint)
    toolbox.register("mutate", tools.mutFlipBit, indpb=0.05)
    toolbox.register("select", tools.selTournament, tournsize=3)

    return toolbox


def specimen_initializer(toolbox, genom_size):
    toolbox.register("action", random.randint, 0, 1)
    toolbox.register("individual", tools.initRepeat, creator.Individual, toolbox.action, n=genom_size)
    toolbox.register("population", tools.initRepeat, list, toolbox.individual)


def solve(toolbox, generations_num, population_initializer):
    if isinstance(population_initializer, int):
        population = toolbox.population(n=population_initializer)
    elif isinstance(population_initializer, str):
        # add capability to load from file
        raise NotImplementedError

    for gen in range(generations_num):
        print(f"Generation {gen+1}/{generations_num}...")
        offspring = algorithms.varAnd(population, toolbox, cxpb=0.5, mutpb=0.1)
        # fits = toolbox.map(toolbox.evaluate, offspring)
        fits = batch_fitness(offspring)
        for fit, ind in zip(fits, offspring):
            ind.fitness.values = fit
        population = toolbox.select(offspring, k=len(population))

    return population
