mod rank;

use futures::executor;
use std::fs;
use std::process::exit;

use crate::comparison::rstarcomp::RstarComparator;
use crate::comparison::Comparator;
use crate::plushie::params::Leniency;
use crate::plushie::{params, Params};
use crate::{common::*, RankArgs};
use rocket::serde::{json::Json, Deserialize};
use rocket::State;

#[derive(Deserialize)]
struct GenomData {
    genome: Vec<u8>,
}

#[post("/batch_fitness", data = "<population>")]
fn batch_fitness(
    judge: &State<RstarComparator>,
    params: &State<Params>,
    population: Json<Vec<GenomData>>,
) -> Json<Vec<f32>> {
    let cmp: &RstarComparator = &judge;
    let ratings: Vec<f32> = population
        .iter()
        .enumerate()
        .map(|(i, specimen)| {
            log::info!("Rating {}/{}...", i + 1, population.len());
            rank::rank(&specimen.genome, params, cmp)
        })
        .collect();
    Json(ratings)
}

#[derive(Deserialize)]
struct FitnessSumData {
    genome: Vec<f32>,
}

#[post("/fitnessSum", data = "<specimen>")]
fn fitness_sum(specimen: Json<FitnessSumData>) -> String {
    let rating: f32 = specimen.genome.iter().sum();
    format!("{}", rating)
}

#[post("/batch_fitness_sum", data = "<population>")]
fn batch_fitness_sum(population: Json<Vec<FitnessSumData>>) -> String {
    let ratings: Vec<f32> = population
        .iter()
        .map(|specimen| specimen.genome.iter().sum())
        .collect();
    format!("{:?}", ratings)
}

fn keep_system_awake() -> Option<keepawake::KeepAwake> {
    keepawake::Builder::default()
        .display(false)
        .idle(true)
        .sleep(true)
        .create()
        .map_or_else(
            |e| {
                log::error!("Couldn't keep awake: {e:?}");
                None
            },
            |x| Some(x),
        )
}

#[tokio::main]
pub async fn start(args: &RankArgs) {
    let _awake = keep_system_awake();

    let basis: Vec<Point> = match fs::read_to_string(&args.goal) {
        Ok(x) => serde_json::from_str(&x).unwrap(),
        Err(e) => {
            log::error!("Could not load pointcloud {:?}: {e}", args.goal);
            exit(1);
        }
    };
    let params: Params = match params::handpicked::get(&args.params) {
        Some(mut p) => {
            p.hook_leniency = Leniency::SkipIncorrect;
            p
        }
        None => {
            log::error!("Unrecognized params {:?}", args.params);
            exit(1);
        }
    };

    println!("Rocket starting");
    let rocket_future = rocket::build()
        .manage(RstarComparator::with_basis(&basis))
        .manage(params)
        .mount("/", routes![batch_fitness, fitness_sum, batch_fitness_sum])
        .launch();
    let _ = executor::block_on(rocket_future);
}
