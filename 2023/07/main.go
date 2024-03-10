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
	type1 handType
	type2 handType
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
		hands = append(hands, hand{cards, bid, getType1(cards), getType2(cards)})
	}
	return hands
}

func getType1(cards cards) handType {
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

func getType2(cards cards) handType {
	counts := make(map[uint8]uint8)
	for _, card := range cards {
		counts[card] += 1
	}
	jCount, jExists := counts[11]
	switch len(counts) {
	case 1:
		// 22222
		// JJJJJ
		return 6
	case 2:
		if jExists {
			// JJJJ2
			// J2222
			// JJ222
			// JJJ22
			return 6
		}
		if slices.Contains(maps.Values(counts), 4) {
			// 22223
			return 5
		}
		// 22233
		return 4
	case 3:
		if slices.Contains(maps.Values(counts), 3) {
			if jExists {
				// J3444
				// JJJ23
				return 5
			}
			// 23444
			return 3
		}
		if jExists {
			if jCount == 2 {
				// JJ233
				return 5
			}
			// J2233
			return 4
		}
		// 23344
		return 2
	case 4:
		if jExists {
			// J2344
			// JJ234
			return 3
		}
		// 23455
		return 1
	case 5:
		if jExists {
			// J2345
			return 1
		}
		// 23456
		return 0
	default:
		log.Fatalln("Unexpected card count")
		return 69
	}
}

func run(input []string) {
	hands := getHands(input)

	sort.Slice(hands, func(i, j int) bool {
		thi := hands[i].type1
		thj := hands[j].type1
		if thi != thj {
			return thi < thj
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

	sort.Slice(hands, func(i, j int) bool {
		thi := hands[i].type2
		thj := hands[j].type2
		if thi != thj {
			return thi < thj
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
