import requests
import json

def fitness(genome):
    # using localhost instead of 127.0.0.1 causes significant delay (prolly IPv6 and DNS shenanigans)
    resp = requests.post("http://127.0.0.1:8001/fitness", json={
        "genome": genome,
    })
    return [float(resp.text)]

def batch_fitness(genomes):
    data = [
        {"genome": genome} for genome in genomes
    ]
    resp = requests.post("http://127.0.0.1:8001/batch_fitness", json=data)
    print(resp.text)
    fitness = json.loads(resp.text)
    fitness = [[fit if fit != None else -float("inf")] for fit in fitness]
    print(fitness)
    return fitness