from .communication import fitness, batch_fitness
import random
import json
from deap import creator, base, tools, algorithms
import os

def init():
    creator.create("FitnessMax", base.Fitness, weights=(1.0,))
    creator.create("Individual", list, fitness=creator.FitnessMax)

    toolbox = base.Toolbox()

    toolbox.register("evaluate", fitness)
    toolbox.register("mate", tools.cxTwoPoint)
    toolbox.register("mutate", tools.mutUniformInt, low=0, up=4, indpb=0.05)
    toolbox.register("select", tools.selTournament, tournsize=3)

    return toolbox

def fresh_specimen(genom_size):
    supported = ["sc", "inc", "dec"]
    return creator.Individual([random.randint(0, len(supported) - 1) for _ in range(genom_size)])


def fresh_population(size, genom_size):
    return [fresh_specimen(genom_size) for _ in range(size)]


def solve(toolbox, generations_num, population):
    path = "population/experiment"
    files = os.listdir(path)
    for file in files:
        file_path = os.path.join(path, file)
        if os.path.isfile(file_path):
            os.remove(file_path)

    for gen in range(generations_num):
        print(f"Generation {gen+1}/{generations_num}...")
        offspring = algorithms.varAnd(population, toolbox, cxpb=0.5, mutpb=0.1)
        with open(f"{path}/generation{gen}.json", "w") as f:
            json.dump(offspring, f)
        fits = batch_fitness(offspring)
        for fit, ind in zip(fits, offspring):
            ind.fitness.values = fit
        population = toolbox.select(offspring, k=len(population))

        unwrapped_fits = [fit[0] for fit in fits]
        print(f"best fitness: {max(unwrapped_fits)}")

    return population
