package main

import (
	"2023/lib"
	"log"
	"slices"
	"strconv"
	"unicode"
)

type Number struct {
	line  int
	start int
	end   int
	value int
}

type Gear struct {
	line  int
	index int
}

func main() {
	input := lib.GetInput(2023, 3)

	var numbers []Number
	var symbols = make(map[int][]int)
	var gears []Gear

	for l, line := range input {
		var number *Number
		for r, run := range line {
			if unicode.IsDigit(run) {
				if number == nil {
					number = new(Number)
					number.line = l
					number.start = r
				}
				number.end = r
			} else {
				if run != '.' {
					symbols[l] = append(symbols[l], r)
					if run == '*' {
						gears = append(gears, Gear{l, r})
					}
				}
				if number == nil {
					continue
				} else {
					substring := line[number.start : number.end+1]
					converted, err := strconv.Atoi(substring)
					if err != nil {
						log.Fatalln("Failed to convert " + substring + " to int")
					}
					number.value = converted
					numbers = append(numbers, *number)
					number = nil
				}
			}
		}
		if number != nil {
			substring := line[number.start : number.end+1]
			converted, err := strconv.Atoi(substring)
			if err != nil {
				log.Fatalln("Failed to convert " + substring + " to int")
			}
			number.value = converted
			numbers = append(numbers, *number)
			number = nil
		}
	}

	sum := 0

number:
	for _, number := range numbers {
		if number.line > 0 {
			for _, symbolIndex := range symbols[number.line-1] {
				if number.start-1 <= symbolIndex && symbolIndex <= number.end+1 {
					sum += number.value
					continue number
				}
			}
		}
		if number.start != 0 && slices.Contains(symbols[number.line], number.start-1) {
			sum += number.value
			continue number
		}
		if slices.Contains(symbols[number.line], number.end+1) {
			sum += number.value
			continue number
		}
		for _, symbolIndex := range symbols[number.line+1] {
			if number.start-1 <= symbolIndex && symbolIndex <= number.end+1 {
				sum += number.value
				continue number
			}
		}
	}

	log.Println("Part 1: " + strconv.Itoa(sum))

	sum = 0
gear:
	for _, gear := range gears {
		var adjacent []Number
		for _, number := range numbers {
			if !(gear.line-1 <= number.line && number.line <= gear.line+1) {
				continue
			}
			if number.start-1 <= gear.index && gear.index <= number.end+1 {
				adjacent = append(adjacent, number)
			}

			if len(adjacent) > 2 {
				continue gear
			}
		}
		if len(adjacent) != 2 {
			continue
		}
		sum += adjacent[0].value * adjacent[1].value
	}

	log.Println("Part 2: " + strconv.Itoa(sum))
}
