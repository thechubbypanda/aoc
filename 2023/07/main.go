package main

import (
	"2023/lib"
	"golang.org/x/exp/maps"
	"log"
	"slices"
	"sort"
	"strconv"
	"strings"
)

func main() {
	test := lib.GetTestInput(7)
	run(test)
	input := lib.GetInput(2023, 7)
	run(input)
}

type cards = [5]uint8
type handType = uint8

type hand struct {
	cards cards
	bid   int
}

func getHands(input []string) []hand {
	var hands []hand
	table := map[rune]uint8{'2': 2, '3': 3, '4': 4, '5': 5, '6': 6, '7': 7, '8': 8, '9': 9, 'T': 10, 'J': 11, 'Q': 12, 'K': 13, 'A': 14}
	for _, line := range input {
		handBid := strings.Fields(line)
		var cards [5]uint8
		for i, c := range handBid[0] {
			cards[i] = table[c]
		}
		bid, _ := strconv.Atoi(handBid[1])
		hands = append(hands, hand{cards, bid})
	}
	return hands
}

func getPart1Type(cards cards) handType {
	counts := make(map[uint8]uint8)
	for _, card := range cards {
		counts[card] += 1
	}
	switch len(counts) {
	case 1:
		return 6
	case 2:
		if slices.Contains(maps.Values(counts), 4) {
			return 5
		} else {
			return 4
		}
	case 3:
		if slices.Contains(maps.Values(counts), 3) {
			return 3
		} else {
			return 2
		}
	case 4:
		return 1
	case 5:
		return 0
	default:
		log.Fatalln("Unexpected card count")
		return 69
	}
}

func getPart2Type(cards cards) handType {
	counts := make(map[uint8]uint8)
	for _, card := range cards {
		counts[card] += 1
	}
	jCount, jExists := counts[13]
	switch len(counts) {
	case 1:
		return 6
	case 2:
		var t uint8 = 69
		if slices.Contains(maps.Values(counts), 4) {
			t = 5
		} else {
			t = 4
		}
		if jExists && jCount != 4 {
			return t + jCount
		}
		return t
	case 3:
		if slices.Contains(maps.Values(counts), 3) {
			if jExists && jCount != 3 {
				return 3 + jCount
			}
			return 3
		} else {
			if jExists {
				if jCount != 2 {
					return 4
				}
				return 5
			}
			return 2
		}
	case 4:
		if jExists {
			if jCount != 1 {
				return 2
			}
			return 3
		}
		return 1
	case 5:
		if jExists {
			return 1
		}
		return 0
	default:
		log.Fatalln("Unexpected card count")
		return 69
	}
}

func run(input []string) {
	hands := getHands(input)

	part1Types := make([]handType, len(hands))
	for i, h := range hands {
		part1Types[i] = getPart1Type(h.cards)
	}
	sort.Slice(hands, func(i, j int) bool {
		if part1Types[i] != part1Types[j] {
			return part1Types[i] < part1Types[j]
		}
		for k := range 5 {
			if hands[i].cards[k] != hands[j].cards[k] {
				return hands[i].cards[k] < hands[j].cards[k]
			}
		}
		return false
	})
	sum := 0
	for i, h := range hands {
		sum += h.bid * (i + 1)
	}
	log.Println("Part 1:", sum)

	for i, h := range hands {
		for j, c := range h.cards {
			if c == 11 {
				hands[i].cards[j] = 1
			}
		}
	}

	part2Types := make([]handType, len(hands))
	for i, h := range hands {
		part2Types[i] = getPart2Type(h.cards)
	}
	sort.Slice(hands, func(i, j int) bool {
		if part1Types[i] != part1Types[j] {
			return part1Types[i] < part1Types[j]
		}
		for k := range 5 {
			if hands[i].cards[k] != hands[j].cards[k] {
				return hands[i].cards[k] < hands[j].cards[k]
			}
		}
		return false
	})
	sum = 0
	for i, h := range hands {
		sum += h.bid * (i + 1)
	}
	log.Println("Part 2:", sum)
}
