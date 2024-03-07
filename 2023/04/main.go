package main

import (
	"2023/lib"
	"log"
	"strconv"
	"strings"
)

type Card struct {
	winning []int
	numbers []int
}

func main() {
	input := lib.GetInput(2023, 4)

	var cards []Card

	for _, line := range input {
		line = strings.Split(line, ": ")[1]
		split := strings.Split(line, " | ")
		winningString := strings.Fields(split[0])
		numbersString := strings.Fields(split[1])
		var winning []int
		for _, s := range winningString {
			val, err := strconv.Atoi(s)
			if err != nil {
				log.Fatalln("Failed to parse string: " + s)
			}
			winning = append(winning, val)
		}
		var numbers []int
		for _, s := range numbersString {
			val, err := strconv.Atoi(s)
			if err != nil {
				log.Fatalln("Failed to parse string: " + s)
			}
			numbers = append(numbers, val)
		}

		cards = append(cards, Card{winning, numbers})
	}

	var cardWins []int

	for _, card := range cards {
		winners := 0
		for _, number := range card.numbers {
			for _, winning := range card.winning {
				if number == winning {
					winners += 1
				}
			}
		}
		cardWins = append(cardWins, winners)
	}

	conversionTable := []int{0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096}

	sum := 0

	for _, wins := range cardWins {
		sum += conversionTable[wins]
	}

	log.Println("Part 1: " + strconv.Itoa(sum))

	cardCounts := make([]int, len(cards))

	for i := range cards {
		cardCounts[i] += 1
		for j := 1; j <= cardWins[i]; j++ {
			cardCounts[i+j] += cardCounts[i]
		}
	}

	sum = 0

	for _, count := range cardCounts {
		sum += count
	}

	log.Println("Part 2: " + strconv.Itoa(sum))
}
