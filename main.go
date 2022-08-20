package main

import (
	"flag"
)

func main() {
	execByIbus := flag.Bool("ibus", false, "This is how IBus will call me")
	execFromTerm := flag.Bool("term", false, "Use when running the input engine directly from the terminal")
	flag.Parse()

	Init(*execByIbus, *execFromTerm)
}
