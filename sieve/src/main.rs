// S -> sieve
// A -> auxiliary.urls
// Z -> known.urls
// Z' -> new-known.urls
// V -> sorting_helper

use std::fs;
use std::io::{Write, Seek};
use std::io::{BufRead, BufReader};

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use std::cmp::Ordering;

const SIEVE_MAX_SIZE: usize = 100;

fn flush_sieve(sieve: &mut Vec<u64>, auxiliary: &mut fs::File) -> Result<(), std::io::Error> {
    // 1
    let mut sorting_helper = (0..sieve.len()).collect::<Vec<usize>>();
    sorting_helper.sort_by(|&i, &j| sieve[i].cmp(&sieve[j]));

    // 2
    let mut unique_urls = vec![false; sieve.len()];
    let mut duplicate_canary = 0;

    for sorted_index in &sorting_helper {
        let url_sign = sieve[*sorted_index];

        if url_sign != duplicate_canary {
            unique_urls[*sorted_index] = true;
        }

        duplicate_canary = url_sign;
    }

    // 3
    let known_urls: Vec<u64> = fs::read_to_string("known.urls")
        .unwrap_or(String::from(""))
        .lines()
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();

    let mut new_known_urls = fs::File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open("new-known.urls")
        .expect("Cannot create temporary file");

    let mut sieve_index: usize = 0;
    let mut known_index: usize = 0;

    let mut discovered_urls = vec![false; sieve.len()];

    loop {
        if sieve_index >= sieve.len() {
            for url in &known_urls[known_index..] {
                write!(new_known_urls, "{:?}\n", *url)?;
            }
            break;
        }

        if known_index >= known_urls.len() {
            for url in &sieve[sieve_index..] {
                write!(new_known_urls, "{:?}\n", *url)?;
                discovered_urls[sorting_helper[sieve_index]] = true;
            }
            break;
        }

        if !unique_urls[sorting_helper[sieve_index]] {
            sieve_index += 1;
            continue;
        }

        let sieve_url_sign = sieve[sorting_helper[sieve_index]];
        let known_url_sign = known_urls[known_index];

        match sieve_url_sign.cmp(&known_url_sign) {
            Ordering::Greater => {
                write!(new_known_urls, "{:?}\n", known_url_sign)?;
                known_index += 1;
            }
            Ordering::Less => {
                write!(new_known_urls, "{:?}\n", sieve_url_sign)?;
                discovered_urls[sorting_helper[sieve_index]] = true;
                sieve_index += 1;
            }
            Ordering::Equal => {
                write!(new_known_urls, "{:?}\n", sieve_url_sign)?;
                known_index += 1;
                sieve_index += 1;
            }
        }
    }

    new_known_urls.flush()?;
    drop(new_known_urls);
    fs::rename("new-known.urls", "known.urls")?;

    // 4
    auxiliary.rewind()?;
    let reader = BufReader::new(auxiliary);
    
    for (index, url) in reader.lines().enumerate() {
        if index >= sieve.len() {
            break;
        }

        if discovered_urls[index] {
            println!("{:?}", url);
        }
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let mut sieve: Vec<u64> = vec![];

    let mut auxiliary = fs::File::options()
        .create(true)
        .write(true)
        .read(true)
        .truncate(true)
        .open("auxiliary.urls")
        .expect("Cannot create auxiliary urls file");

    let seed = fs::read_to_string("seed.urls")
        .expect("No seed found, provide a seed.urls file");

    let mut hasher = DefaultHasher::new();

    for url in seed.lines() {
        url.hash(&mut hasher);

        sieve.push(hasher.finish());
        write!(auxiliary, "{}\n", url)?;
    
        if sieve.len() >= SIEVE_MAX_SIZE {
            auxiliary.flush()?;
            flush_sieve(&mut sieve, &mut auxiliary)?;

            auxiliary.rewind()?;
            auxiliary.set_len(0)?;

            while !sieve.is_empty() {
                sieve.pop();
            }
        }

    }
    
    fs::remove_file("auxiliary.urls")?;
    fs::remove_file("known.urls")?;

    Ok(())
}