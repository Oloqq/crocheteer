from .communication import fitness, batch_fitness
import random
import json
from deap import creator, base, tools, algorithms
import os

supported = ["sc", "inc", "dec"]
MIN_GENE = 0
MAX_GENE = len(supported) - 1


def init():
    creator.create("FitnessMax", base.Fitness, weights=(1.0,))
    creator.create("Individual", list, fitness=creator.FitnessMax)

    toolbox = base.Toolbox()

    toolbox.register("evaluate", fitness)
    toolbox.register("mate", tools.cxTwoPoint)
    toolbox.register("mutate", tools.mutUniformInt, low=MIN_GENE, up=MAX_GENE, indpb=0.05)
    toolbox.register("select", tools.selTournament, tournsize=3)

    return toolbox

def fresh_specimen(genom_size):
    return creator.Individual([random.randint(MIN_GENE, MAX_GENE) for _ in range(genom_size)])
    # return creator.Individual([0 for _ in range(genom_size)])


def fresh_population(size, genom_size):
    return [fresh_specimen(genom_size) for _ in range(size)]

def load_population(filepath):
    with open(filepath) as f:
        data = json.load(f)
    population = [creator.Individual(dude) for dude in data]
    return population

def solve(experiment_path, toolbox, generations_num, population, starting_generation):
    for gen in range(starting_generation, generations_num):
        print(f"Generation {gen+1}/{generations_num}...")
        offspring = algorithms.varAnd(population, toolbox, cxpb=0.5, mutpb=0.1)
        with open(f"{experiment_path}/generation{gen}.json", "w") as f:
            json.dump(offspring, f)
        fits = batch_fitness(offspring)
        for fit, ind in zip(fits, offspring):
            ind.fitness.values = fit
        population = toolbox.select(offspring, k=len(population))

        unwrapped_fits = [fit[0] for fit in fits]
        print(f"best fitness: {max(unwrapped_fits)}")

    return population
