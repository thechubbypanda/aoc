package main

import (
	"2023/lib"
	"log"
	"strconv"
	"strings"
)

func main() {
	input := lib.GetInput(2023, 1)

	numbers := map[string]int{"0": 0, "1": 1, "2": 2, "3": 3, "4": 4, "5": 5, "6": 6, "7": 7, "8": 8, "9": 9}
	sum := 0
	for _, line := range input {
		sum += calculate(line, numbers)
	}
	log.Println("Part 1: " + strconv.Itoa(sum))

	numbers = map[string]int{"zero": 0, "one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9, "0": 0, "1": 1, "2": 2, "3": 3, "4": 4, "5": 5, "6": 6, "7": 7, "8": 8, "9": 9}
	sum = 0
	for _, line := range input {
		sum += calculate(line, numbers)
	}
	log.Println("Part 2: " + strconv.Itoa(sum))
}

func calculate(line string, numbers map[string]int) int {
	first := -1
	last := -1
	for {
		for key, val := range numbers {
			if strings.HasPrefix(line, key) {
				if first == -1 {
					first = val
				}
				last = val
				break
			}
		}
		line = string([]rune(line)[1:])
		if line == "" {
			break
		}
	}
	return first*10 + last
}
