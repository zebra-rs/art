use art::*;
use ipnet::Ipv4Net;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time;

#[test]
fn ipv4_route_random1() {
    let now = time::Instant::now();
    let mut top = ArtRoot::<Ipv4Net, u32>::new_ipv4_table();

    let file = File::open("tests/data/v4routes-random1.txt").unwrap();
    let bufferd = BufReader::new(file);

    for line in bufferd.lines() {
        let line = line.unwrap();
        let prefix: Ipv4Net = line.parse().unwrap();
        top.route_ipv4_add(prefix, 0);
    }
    assert_eq!(top.iter().count(), 569770);

    let file = File::open("tests/data/v4routes-random1.txt").unwrap();
    let bufferd = BufReader::new(file);

    for line in bufferd.lines() {
        let line = line.unwrap();
        let prefix: Ipv4Net = line.parse().unwrap();
        top.route_ipv4_delete(prefix);
    }

    assert_eq!(top.iter().count(), 0);
    println!("ipv4_route_random1 {:?}", now.elapsed());
}

#[test]
fn ipv4_route_random1_lookup_exact() {
    let now = time::Instant::now();
    let mut top = ArtRoot::<Ipv4Net, i32>::new_ipv4_table();

    let file = File::open("tests/data/v4routes-random1.txt").unwrap();
    let bufferd = BufReader::new(file);

    for line in bufferd.lines() {
        let line = line.unwrap();
        let prefix: Ipv4Net = line.parse().unwrap();
        top.route_ipv4_add(prefix, 0);
    }
    assert_eq!(top.iter().count(), 569770);

    let file = File::open("tests/data/v4routes-random1.txt").unwrap();
    let bufferd = BufReader::new(file);

    for line in bufferd.lines() {
        let line = line.unwrap();
        let prefix: Ipv4Net = line.parse().unwrap();
        let result = top.route_ipv4_lookup_exact(prefix);
        assert!(result.is_some());
    }
    println!("ipv4_route_random1_lookup_exact {:?}", now.elapsed());
}

// #[test]
// fn ipv6_route_random1() {
//     let now = time::Instant::now();
//     let mut top = ArtRoot::<Ipv4Net, i32>::new_ipv6_table();

//     let file = File::open("tests/data/v6routes-random1.txt").unwrap();
//     let bufferd = BufReader::new(file);

//     for line in bufferd.lines() {
//         let line = line.unwrap();
//         top.route_ipv6_add(&line, 0);
//     }
//     assert_eq!(top.iter().count(), 24470);

//     let file = File::open("tests/data/v6routes-random1.txt").unwrap();
//     let bufferd = BufReader::new(file);

//     for line in bufferd.lines() {
//         let line = line.unwrap();
//         top.route_ipv6_delete(&line);
//     }

//     assert_eq!(top.iter().count(), 0);
//     println!("ipv6_route_random1 {:?}", now.elapsed());
// }

// #[test]
// fn ipv6_route_random1_lookup_exact() {
//     let now = time::Instant::now();
//     let mut top = ArtRoot::<Ipv4Net, i32>::new_ipv6_table();

//     let file = File::open("tests/data/v6routes-random1.txt").unwrap();
//     let bufferd = BufReader::new(file);

//     for line in bufferd.lines() {
//         let line = line.unwrap();
//         top.route_ipv6_add(&line, 0);
//     }
//     assert_eq!(top.iter().count(), 24470);

//     let file = File::open("tests/data/v6routes-random1.txt").unwrap();
//     let bufferd = BufReader::new(file);

//     for line in bufferd.lines() {
//         let line = line.unwrap();
//         let result = top.route_ipv6_lookup_exact(&line);
//         assert!(result.is_some());
//     }
//     println!("ipv6_route_random1_lookup_exact {:?}", now.elapsed());
// }
