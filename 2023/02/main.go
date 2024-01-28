package main

import (
	"2023/lib"
	"log"
	"strconv"
	"strings"
)

func main() {
	input := lib.GetInput(2023, 2)
	var games [][]map[string]int
	for _, line := range input {
		grabsString := strings.Split(strings.Split(line, ": ")[1], "; ")
		var grabs []map[string]int
		for _, grabString := range grabsString {
			var grab = map[string]int{}
			colors := strings.Split(grabString, ", ")
			for _, color := range colors {
				cubesByColor := strings.Split(color, " ")
				grab[cubesByColor[1]], _ = strconv.Atoi(cubesByColor[0])
			}
			grabs = append(grabs, grab)
		}
		games = append(games, grabs)
	}

	rules := map[string]int{"red": 12, "green": 13, "blue": 14}

	sum := 0

games:
	for i, game := range games {
		for _, grab := range game {
			for color, count := range grab {
				if count > rules[color] {
					continue games
				}
			}
		}
		sum += i + 1
	}
	log.Println("Part 1: " + strconv.Itoa(sum))

	sum = 0
	for _, game := range games {
		rules = map[string]int{"red": 0, "green": 0, "blue": 0}
		for _, grab := range game {
			for color, count := range grab {
				rules[color] = Max(rules[color], count)
			}
		}
		power := 1
		for _, count := range rules {
			power *= count
		}
		sum += power
	}

	log.Println("Part 2: " + strconv.Itoa(sum))
}

func Max(x int, y int) int {
	if x < y {
		return y
	}
	return x
}
