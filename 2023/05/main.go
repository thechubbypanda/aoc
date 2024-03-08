package main

import (
	"2023/lib"
	"log"
	"math"
	"strconv"
	"strings"
)

type Range struct {
	start int
	end   int
}

type Map = []Range

type Rule struct {
	src    Range
	dst    Range
	change int
}

type Ruleset = []Rule

func main() {
	test := lib.GetTestInput(5)
	run(test)
	input := lib.GetInput(2023, 5)
	run(input)
}

func rulesets(input []string) []Ruleset {
	var rulesets []Ruleset
	var ruleset *Ruleset
	for i := 1; i < len(input); i++ {
		line := input[i]
		if strings.TrimSpace(line) == "" {
			if ruleset != nil {
				rulesets = append(rulesets, *ruleset)
			}
			ruleset = &[]Rule{}
			continue
		}
		if strings.Contains(line, ":") {
			continue
		}
		fields := strings.Fields(line)
		var ruleData [3]int
		for j, s := range fields {
			val, err := strconv.Atoi(s)
			if err != nil {
				log.Fatalln("Failed to convert string: " + s)
			}
			ruleData[j] = val
		}
		dstStart := ruleData[0]
		srcStart := ruleData[1]
		rng := ruleData[2] - 1
		*ruleset = append(*ruleset, Rule{
			Range{
				srcStart,
				srcStart + rng,
			},
			Range{
				dstStart,
				dstStart + rng,
			},
			dstStart - srcStart,
		})
	}
	if ruleset == nil {
		log.Fatalln("Unexpected nil ruleset")
		return nil
	}
	rulesets = append(rulesets, *ruleset)
	ruleset = nil
	return rulesets
}

func part1Seeds(input []string) Map {
	var m Map
	for _, s := range strings.Fields(input[0])[1:] {
		val, err := strconv.Atoi(s)
		if err != nil {
			log.Fatalln("Failed to convert string: " + s)
		}
		m = append(m, Range{val, val})
	}
	return m
}

func part2Seeds(input []string) Map {
	seeds := part1Seeds(input)
	var m Map
	for i := 0; i < len(seeds)/2; i++ {
		m = append(m, Range{seeds[i*2].start, seeds[i*2].start + seeds[i*2+1].start - 1})
	}
	return m
}

func translate(m Map, ruleset Ruleset) Map {
	for i := 0; i < len(m); i++ {
		for _, rule := range ruleset {
			rng := m[i]
			// Rule not overlapped with range
			if rule.src.end < rng.start || rule.src.start > rng.end {
				continue
			}

			// Rule completely encapsulates range
			if rule.src.start <= rng.start && rule.src.end >= rng.end {
				// Update the whole range
				m[i].start += rule.change
				m[i].end += rule.change
				break
			}
			// Rule overlaps lower side of range
			if rule.src.start <= rng.start && rule.src.end < rng.end {
				// Grab the unchanged portion
				unchanged := Range{rule.src.end + 1, rng.end}
				// Create new range in both maps so that it gets checked and updated in future
				m = append(m, unchanged)
				m = append(m, unchanged)
				// Update the changed portion of the range
				m[i].start += rule.change
				m[i].end = rule.dst.end
				break
			}
			// Rule overlaps upper side of range
			if rule.src.start > rng.start && rule.src.end >= rng.end {
				// Grab the unchanged portion
				unchanged := Range{rng.start, rule.src.start - 1}
				// Create new range in both maps so that it gets checked and updated in future
				m = append(m, unchanged)
				m = append(m, unchanged)
				// Update the changed portion of the range
				m[i].start = rule.dst.start
				m[i].end += rule.change
				break
			}
			// Rule is encapsulated by range
			if rule.src.start > rng.start && rule.src.end < rng.end {
				// Grab the lower unchanged portion
				unchangedLower := Range{rng.start, rule.src.start - 1}
				// Move it to a new range
				m = append(m, unchangedLower)
				m = append(m, unchangedLower)
				// Grab the upper unchanged portion
				unchangedUpper := Range{rule.src.end + 1, rng.end}
				// Move it to a new range
				m = append(m, unchangedUpper)
				m = append(m, unchangedUpper)
				// Update the changed portion of the range
				m[i].start = rule.dst.start
				m[i].end = rule.dst.end
				break
			}
			log.Fatalln("We should have hit a case")
		}
	}
	return m
}

func closest(m Map) int {
	closest := math.MaxInt
	for _, r := range m {
		closest = min(r.start, closest)
	}
	return closest
}

func run(input []string) {
	rulesets := rulesets(input)

	m := part1Seeds(input)

	for _, rs := range rulesets {
		m = translate(m, rs)
	}

	log.Println("Part 1:", closest(m))

	m = part2Seeds(input)

	for _, rs := range rulesets {
		m = translate(m, rs)
	}

	log.Println("Part 2:", closest(m))
}
