package main

import (
	"2023/lib"
	"log"
	"strconv"
	"strings"
)

func main() {
	test := lib.GetTestInput(6)
	run(test)
	input := lib.GetInput(2023, 6)
	run(input)
}

type Race struct {
	time     int
	distance int
}

func wins(race Race, timeHeld int) bool {
	distanceTraveled := (race.time - timeHeld) * timeHeld
	return distanceTraveled > race.distance
}

func run(input []string) {
	times := strings.Fields(input[0])[1:]
	distances := strings.Fields(input[1])[1:]

	var races []Race
	for i, time := range times {
		t, _ := strconv.Atoi(time)
		d, _ := strconv.Atoi(distances[i])
		races = append(races, Race{t, d})
	}

	product := 1
	for _, race := range races {
		count := 0
		for timeHeld := range race.time {
			distanceTraveled := (race.time - timeHeld) * timeHeld
			if distanceTraveled > race.distance {
				count++
			}
		}
		product *= count
	}

	log.Println("Part 1:", product)

	t, _ := strconv.Atoi(strings.Join(times, ""))
	d, _ := strconv.Atoi(strings.Join(distances, ""))
	race := Race{t, d}

	top := race.time / 2
	timeHeld := top / 2
	bottom := 0

	for {
		if wins(race, timeHeld) {
			if wins(race, timeHeld+1) {
				if wins(race, timeHeld-1) {
					top = timeHeld
					timeHeld = (bottom + timeHeld) / 2
				} else {
					break
				}
			} else {
				log.Fatalln("Won with", timeHeld, "but not with", timeHeld+1)
			}
		} else {
			if wins(race, timeHeld+1) {
				timeHeld += 1
				break
			} else {
				bottom = timeHeld
				timeHeld = (top + timeHeld) / 2
			}
		}
	}

	waysToWin := race.time - timeHeld*2 + 1

	log.Println("Part 2:", waysToWin)
}
