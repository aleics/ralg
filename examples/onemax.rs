// To execute this example run: cargo run --example onemax
extern crate rsmath as r;
extern crate rand;

use rand::Rng;
use r::algebra::matrix::*;
use r::algebra::vector::*;

/// Onemax is a basic problem of evolutionary algorithms. The aim of the problem is to find a individuals
/// where its genes are all '1'. This is the first algorithm that I learned, when I was introduced to the
/// world of the Genetics Algorithms.
/// This algorithm is divided in different parts:
/// * Initialization of variables: the needed variables and the initial population will be initializated.
/// * Fitness calculation: the fitness will be calculated for the given problem (in this case the number of '1').
/// * Tournament selection: a combination of individuals will "fight" each other. The ones with the best fitness
///   will be selected for the next step. In total two arrays of winners will be generated
/// * Crossover: using the winners (also called parents) the new population will be created. The individuals of the
///   new population will be created as a product of the combination of the parents.
/// * Mutation: if decided, a gene of the new population's individuals will be modified.
/// * Elitism: to preserve that the best individual of the last population won't be deleted, will be saved on the
///   new population.

fn main() {
    // initialization of variables
    let pop_size: usize = 20;
    let n_genes: usize = 10;
    let max_gens: usize = 500;

    let mut max_fitness_arr: Vec<u32> = Vec::new();
    let mut median_fitness_arr: Vec<f64> = Vec::new();

    let mut pop = Matrix::<u32>::random(pop_size, n_genes, &[0, 1]);

    println!("initial population: \n{}", pop);

    // iterate for the generations
    for gen in 0..max_gens {

        // get the fitness: number of '1'
        let mut fitness = Vector::<u32>::new();
        for gene in pop.row_iter() {
            let sum = gene.iter().fold(0u32, |sum, &el| sum + el);
            fitness.push(sum);
        }

        // get the best indiv and its fitness
        let (max, idx_max) = fitness.max();
        max_fitness_arr.push(max);

        // save the best individual
        let best_indiv = pop.row(idx_max).unwrap().clone();

        // calculate the median fitness of the current generation
        let median = fitness.median();
        median_fitness_arr.push(median);

        if (max as usize) == n_genes {
            println!("value found in {} generations", gen);
            println!("best fitness = {}", max);
            println!("median fitness = {}", median);
            println!("final pop:\n{}", pop);
            break;
        }

        // Tournament selection
        let matchup_a = Matrix::<u32>::random(pop_size, 2, &[0, (pop_size - 1) as u32]);
        let matchup_b = Matrix::<u32>::random(pop_size, 2, &[0, (pop_size - 1) as u32]);

        let parent_a = get_parent(&matchup_a, &fitness);
        let parent_b = get_parent(&matchup_b, &fitness);

        // Crossover
        let mut new_pop = x_over(&pop, &parent_a, &parent_b);

        // Mutation
        mutation(&mut new_pop);

        pop = new_pop.clone();

        // Elitism
        pop.set_row(0, &best_indiv);
    }
}

fn get_parent(matchup: &Matrix<u32>, fitness: &Vector<u32>) -> Vector<u32> {
    if matchup.ncols() != 2 {
        panic!("number of rows of matchup must be 2");
    }

    let mut parent = Vector::<u32>::new();
    for row in matchup.row_iter() {
        let idx_first = row[0] as usize;
        let idx_second = row[1] as usize;

        if fitness.el(idx_first) >= fitness.el(idx_second) {
            parent.push(row[0]);
        } else {
            parent.push(row[1]);
        }
    }
    parent
}

fn x_over(pop: &Matrix<u32>, parent_a: &Vector<u32>, parent_b: &Vector<u32>) -> Matrix<u32> {
    let mut new_pop = Matrix::<u32>::new();
    let do_xover = Vector::<u32>::random(pop.nrows(), &[0, 1]);
    for i in 0..pop.nrows() {
        let mut new_indiv: Vec<u32> = Vec::new();
        if do_xover.el(i) == 1 {
            let crosspoint: usize = rand::thread_rng().gen_range(0, pop.ncols() + 1);
            for j in 0..pop.ncols() {
                if j <= crosspoint {
                    let idx = parent_a.el(i) as usize;
                    new_indiv.push(pop.get_element(idx, j));
                } else {
                    let idx = parent_b.el(i) as usize;
                    new_indiv.push(pop.get_element(idx, j));
                }
            }
        } else {
            let idx = parent_a.el(i) as usize;
            new_indiv = pop.row(idx).unwrap().clone();
        }
        new_pop.push_row(new_indiv);
    }
    new_pop
}

fn mutation(pop: &mut Matrix<u32>) {
    let do_mutation = Matrix::<u32>::random(pop.nrows(), pop.ncols(), &[0, 1]);
    for i in 0..pop.nrows() {
        for j in 0..pop.ncols() {
            if do_mutation.get_element(i, j) == 1 {
                if pop.get_element(i, j) == 1 {
                    pop.set_element(i, j, &0u32);
                } else {
                    pop.set_element(i, j, &1u32);
                }
            }
        }
    }
}
