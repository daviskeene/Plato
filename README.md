# Plato - A reduced version of the ls command

## CS 196 Project
For CS196 (Freshman Honors), we were tasked with creating Plato, a rust-built program that would behave in the same way as LS. This was a way for us to get comfortable writing CLI's with Rust, and to get involved with more of the low level things that Rust has to offer.

## Getting Started

Once you've cloned or forked the repo and downloaded it to your local machine, `cd` into the root directory and run:

```
cargo build
```
This will build the `target/` folder and allow you to use the plato script.

Example usage below.

## Usage
```Bash
davis-MacBook-Pro-6:plato davis$ ./target/debug/plato
Cargo.toml	target	Cargo.lock	README.md	src	
davis-MacBook-Pro-6:plato davis$ ./target/debug/plato -l
davis	247	 Aug 12 23:22	Cargo.toml
davis	0	 Sep 16 15:38	target
davis	12349	 Aug 12 23:29	Cargo.lock
davis	625	 Aug 19 00:27	README.md
davis	0	 Sep 17 15:47	src
davis-MacBook-Pro-6:plato davis$ ./target/debug/plato -lh
davis	247 B	 Aug 12 23:22	Cargo.toml
davis	0 B	 Sep 16 15:38	target
davis	12.35 kB	 Aug 12 23:29	Cargo.lock
davis	625 B	 Aug 19 00:27	README.md
davis	0 B	 Sep 17 15:47	src
davis-MacBook-Pro-6:plato davis$ ./target/debug/plato -lha
davis	6 B	 Aug 19 01:52	.hello
davis	247 B	 Aug 12 23:22	Cargo.toml
davis	0 B	 Sep 16 15:38	target
davis	12.35 kB	 Aug 12 23:29	Cargo.lock
davis	625 B	 Aug 19 00:27	README.md
davis	0 B	 Sep 16 17:34	.git
davis	0 B	 Sep 17 15:47	src
davis-MacBook-Pro-6:plato davis$ ./target/debug/plato -a
.hello	Cargo.toml	target	Cargo.lock	README.md	.git	src	
davis-MacBook-Pro-6:plato davis$ ./target/debug/plato src/
lib	main.rs	
davis-MacBook-Pro-6:plato davis$ 
```

