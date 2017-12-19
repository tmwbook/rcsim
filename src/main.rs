use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[macro_use]
extern crate clap;

struct CacheLine {
    valid: bool,
    tag: i32,
}

struct CacheSet {
    lines: Vec<CacheLine>,
}

struct ModelCache {
    opts: Options,
    stats: Statistics,
    sets: Vec<CacheSet>,
}

struct Options {
    sets:   i32,
    lines:  i32,
    blocks: i32,
}

struct Statistics {
    misses:    i32,
    hits:      i32,
    evictions: i32,
}

fn main() {
    let matches = clap_app!(csim =>
        (version: "1.0")
        (author: "Tom White <twhite@wpi.edu>")
        (@arg SETS: +required "Number of sets in cache")
        (@arg LINES: +required "Number of lines in set")
        (@arg BLOCKS: +required "Number of blocks in line")
        (@arg FILE: +required "Trace file location")
    ).get_matches();


    let sets:   i32 = matches.value_of("SETS").unwrap().parse().unwrap();
    let lines:  i32 = matches.value_of("LINES").unwrap().parse().unwrap();
    let blocks: i32 = matches.value_of("BLOCKS").unwrap().parse().unwrap();

    let mut stats = Statistics {misses: 0, hits: 0, evictions: 0};
    let opts = Options{sets, lines, blocks};

    /*
    We have two options:
        1) Wrap structs in an `Option` struct to not need to initialize everything on startup
        2) Init everything and never check if the things exist
    */

    let mut cache: ModelCache = make_cache(opts, stats);

    read_trace(matches.value_of("FILE").unwrap(), &cache);
    
    println!("Hits: {} Misses: {} Evictions: {}", cache.stats.hits,
                                                  cache.stats.misses,
                                                  cache.stats.evictions);
}

fn make_cache(options: Options, statistics: Statistics) -> ModelCache {
    let mut sets: Vec<CacheSet> = Vec::new();
    for _ in 0..options.sets {
        let mut lines: Vec<CacheLine> = Vec::new();
        for __ in 0..options.lines {
            let mut line = CacheLine{valid: false, tag: 0};
        }
        sets.push(CacheSet {lines: lines});
    }
    ModelCache {opts: options, stats: statistics, sets: sets}
}

fn read_trace(file_loc: &str, cache: &ModelCache) {
    let file = File::open(file_loc).unwrap();
    let file = BufReader::new(file);

    for line in file.lines() {
        println!("{}", line.unwrap());
    }
}