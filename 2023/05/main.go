package main

import (
	"2023/lib"
	"log"
	"math"
	"strconv"
	"strings"
)

type Rule struct {
	destStart   int
	sourceStart int
	valueRange  int
	destEnd     int
	sourceEnd   int
}

type Range struct {
	start int
	end   int
}

func main() {
	input := lib.GetInput(2023, 5)

	var seeds []int
	for _, s := range strings.Fields(input[0])[1:] {
		val, err := strconv.Atoi(s)
		if err != nil {
			log.Fatalln("Failed to convert string: " + s)
		}
		seeds = append(seeds, val)
	}

	var ruleSets [][]Rule
	var ruleSet *[]Rule
	for i := 1; i < len(input); i++ {
		line := input[i]
		if strings.TrimSpace(line) == "" {
			if ruleSet != nil {
				ruleSets = append(ruleSets, *ruleSet)
			}
			ruleSet = &[]Rule{}
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
		*ruleSet = append(*ruleSet, Rule{
			ruleData[0],
			ruleData[1],
			ruleData[2],
			ruleData[0] + ruleData[2] - 1,
			ruleData[1] + ruleData[2] - 1,
		})
	}

	ruleSets = append(ruleSets, *ruleSet)
	ruleSet = nil

	var values = make([]int, len(seeds))
	copy(values, seeds)
	for _, rs := range ruleSets {
	value:
		for i, value := range values {
			for _, rule := range rs {
				diff := value - rule.sourceStart
				if 0 <= diff && diff < rule.valueRange {
					values[i] = rule.destStart + diff
					continue value
				}
			}
		}
	}

	m := math.MaxInt
	for _, v := range values {
		m = min(m, v)
	}

	log.Println("Part 1: " + strconv.Itoa(m))

	var ranges []Range
	for i := 0; i < len(seeds)/2; i++ {
		ranges = append(ranges, Range{seeds[i*2], seeds[i*2] + seeds[i*2+1] - 1})
	}
	for _, rs := range ruleSets {
		touched := make(map[int]struct{})
		for i := 0; i < len(ranges); i++ {
			for _, rule := range rs {
				_, ok := touched[i]
				if ok {
					continue
				}
				if ranges[i].end < rule.sourceStart || ranges[i].start > rule.sourceEnd {
					continue
				}
				change := rule.destStart - rule.sourceStart
				if rule.sourceStart <= ranges[i].start && rule.sourceEnd >= ranges[i].end {
					ranges[i].start += change
					ranges[i].end += change
					touched[i] = struct{}{}
					continue
				}
				if rule.sourceStart <= ranges[i].start && rule.sourceEnd < ranges[i].end {
					ranges = append(ranges, Range{ranges[i].start + change, rule.destEnd})
					ranges[i].start = rule.sourceEnd + 1
					touched[i] = struct{}{}
					continue
				}
				if rule.sourceStart > ranges[i].start && rule.sourceEnd >= ranges[i].end {
					ranges = append(ranges, Range{rule.destStart, ranges[i].end + change})
					ranges[i].end = rule.sourceStart - 1
					touched[i] = struct{}{}
					continue
				}
				if rule.sourceStart > ranges[i].start && rule.sourceEnd < ranges[i].end {
					ranges = append(ranges, Range{rule.destStart, rule.destEnd})
					ranges = append(ranges, Range{rule.sourceEnd + 1, ranges[i].end})
					touched[len(ranges)-1] = struct{}{}
					ranges[i].end = rule.sourceStart - 1
					touched[i] = struct{}{}
					continue
				}
				log.Fatalln("Bad")
			}
		}
	}

	m = math.MaxInt
	for _, r := range ranges {
		m = min(r.start, m)
	}
	log.Println("Part 2: " + strconv.Itoa(m))
}
