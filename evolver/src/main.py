from . import genetic, communication
from deap import tools
import json
import random
import click
import os

ACTIONS_NUM_FOR_GRZIB = 133 # 135 - 2 bcs skipping MR and FO

def latest_generation(path):
    files = os.listdir(path)
    file = next(reversed(files))
    while not file.endswith(".json"):
        file = next(files)
    generation = int(file.partition(".json")[0].partition("generation")[2])
    r = f"{path}/generation{generation}.json"
    return r, generation

def refresh_directory(path):
    if os.path.exists(path):
        files = os.listdir(path)
        for file in files:
            file_path = os.path.join(path, file)
            if os.path.isfile(file_path):
                os.remove(file_path)
    else:
        os.mkdir(path)

@click.command()
@click.argument("experiment_name")
@click.option("-f", "--fresh", is_flag=True)
@click.option("-s", "--seed", type=click.INT)
def train(experiment_name, fresh, seed):
    experiment_path = f"population/experiments/{experiment_name}"
    if seed:
        random.seed(seed)

    tb = genetic.init()
    generation = 0

    if fresh:
        refresh_directory(experiment_path)
        population = genetic.fresh_population(size=50, genom_size=ACTIONS_NUM_FOR_GRZIB)
    else:
        if not os.path.exists(experiment_path):
            print(f"Experiment '{experiment_name}' does not exist. Maybe you meant to run with -f?")
            exit(1)
        population_file, generation = latest_generation(f"{experiment_path}")
        population = genetic.load_population(population_file)

    solutions = genetic.solve(experiment_path, tb, generations_num=1000, population=population, starting_generation=generation)
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