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
        let line = line.unwrap();
        if line.chars().nth(0).unwrap() != ' ' {
            println!("SKIPPED LINE!");
            continue;
        }
        handle_instruction(&line, cache);
    }
}

fn handle_instruction(line: &String, cache: &ModelCache) {
    // Really ugly parsing ahead
    let inst: char = line.chars().nth(1).unwrap();
    let (_, end): (&str, &str) = line.split_at(4);
    let addr: &str = end.split(',').nth(0).unwrap();

    // Get the full address in dec
    let addr: u32 = u32::from_str_radix(addr, 16).unwrap();

    // Do some bitwise to get the different parts of the destination

    /*
        C code to do this stuff
        
        int mask = -1;
        mask = mask << (cache->opts->blockBits);
        //block = (~mask & addr);

        int setMask = -1;
        setMask = setMask << (cache->opts->setBits);
        setMask = ~setMask;
        setMask = setMask << (cache->opts->blockBits);
        set = setMask & addr;
        set = set >> (cache->opts->blockBits);

        mask = mask << (cache->opts->setBits);
        tag = mask & addr;
        tag = tag >> (cache->opts->setBits + cache->opts->blockBits);
     */

    let mut set_mask = std::u32::MAX;
    set_mask = set_mask << (cache.opts.sets);
    set_mask = set_mask.wrapping_neg() - 1;
    set_mask = set_mask << (cache.opts.blocks);
    let mut set = set_mask & addr;
    set = set >> (cache.opts.blocks);

    let mut tag_mask = std::u32::MAX;
    tag_mask = tag_mask << (cache.opts.blocks);
    tag_mask = tag_mask << (cache.opts.sets);
    let mut tag = tag_mask & addr;
    tag = tag >> (cache.opts.sets + cache.opts.blocks);

    println!("{} @ {}: Set {} Tag {}", inst, addr, set, tag);
}