# SaveBreaker
A universal save-game file analysis framework written in Rust. Under construction :)

commands:
```
  help                 show this help
  load <name> <path>   load a save into memory
  list                 list loaded saves
  peek <name>          prints the first 80 characters of loaded save
  diff <a> <b>         compare two loaded saves (placeholder)
  exit | quit          leave the REPL
```

This program runs in a REPL console. To run this program, simply run `run.sh`:
```
chmod +x run.sh
./run.sh
```
