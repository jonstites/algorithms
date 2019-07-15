extern crate algorithms;
use algorithms::data::data::graph::Graph;

#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use itertools::Itertools;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("algorithms")
        .about("Practice with algorithms and data structures")
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("bfs")
                .about("Breadth First Search on a graph.")
                .arg(
                    Arg::with_name("input")
                        .help("The file with the input graph")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("dfs")
                .about("Depth First Search on a graph.")
                .arg(
                    Arg::with_name("input")
                        .help("The file with the input graph")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("bfs") {
        let input = matches.value_of("input").unwrap();
        let (graph, source, destination) = parse_graph_file(input)?;
        println!("{:?}", graph.bfs(source, destination));
    }

    if let Some(matches) = matches.subcommand_matches("dfs") {
        let input = matches.value_of("input").unwrap();
        let (graph, source, destination) = parse_graph_file(input)?;
        println!("{:?}", graph.dfs(source, destination));
    }
    Ok(())
}

fn parse_graph_file(input: &str) -> Result<(Graph<i32>, usize, usize), std::io::Error> {
    let data = std::fs::read_to_string(input)?;
    let mut lines = data.lines();
    let num_nodes = lines
        .next()
        .expect("Bad graph file")
        .parse::<usize>()
        .expect("Could not parse number of nodes");

    let nodes = vec![0; num_nodes];

    let edges_str = lines.next().expect("Bad graph file");

    let edges: Vec<(usize, usize)> = edges_str
        .split(|c| c == ']' || c == ',' || c == '[')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>())
        .map(|s| s.unwrap())
        .tuples()
        .collect();

    let source = lines
        .next()
        .expect("Bad graph file")
        .parse::<usize>()
        .expect("Could not parse source id");

    let destination = lines
        .next()
        .expect("Bad graph file")
        .parse::<usize>()
        .expect("Could not parse source id");

    let graph = Graph::new_unweighted(nodes, edges);
    Ok((graph, source, destination))
}
