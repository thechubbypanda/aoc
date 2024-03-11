package main

import (
	"2023/lib"
	"log"
	"slices"
	"strconv"
	"strings"
)

func main() {
	test := lib.GetTestInput(9)
	run(test)
	input := lib.GetInput(2023, 9)
	run(input)
}

func parseHistories(input []string) [][]int {
	histories := make([][]int, len(input))
	for i, line := range input {
		fields := strings.Fields(line)
		histories[i] = make([]int, len(fields))
		for j, field := range fields {
			val, err := strconv.Atoi(field)
			if err != nil {
				log.Fatalln("Can't parse: ", field)
			}
			histories[i][j] = val
		}
	}
	return histories
}

func allZero(history []int) bool {
	for _, v := range history {
		if v != 0 {
			return false
		}
	}
	return true
}

func differences(history []int) []int {
	diffs := make([]int, len(history)-1)
	for i := 1; i < len(history); i++ {
		diffs[i-1] = history[i] - history[i-1]
	}
	return diffs
}

func interpolateRecurse(interpolations [][]int) [][]int {
	current := interpolations[len(interpolations)-1]
	if allZero(current) {
		return interpolations
	}
	next := differences(current)
	return interpolateRecurse(append(interpolations, next))
}

func interpolate(history []int) [][]int {
	interpolations := make([][]int, 1)
	interpolations[0] = history
	output := interpolateRecurse(interpolations)
	slices.Reverse(output)
	return output
}

func run(input []string) {
	histories := parseHistories(input)

	sum := 0
	for _, h := range histories {
		interpolations := interpolate(h)

		nextValue := 0
		for i := 1; i < len(interpolations); i++ {
			nextValue = interpolations[i][len(interpolations[i])-1] + nextValue
		}

		sum += nextValue
	}
	log.Println("Part 1:", sum)

	sum = 0
	for _, h := range histories {
		interpolations := interpolate(h)

		prevValue := 0
		for i := 1; i < len(interpolations); i++ {
			prevValue = interpolations[i][0] - prevValue
		}

		sum += prevValue
	}
	log.Println("Part 2:", sum)
}
