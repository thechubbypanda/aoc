package main

import (
	"2023/lib"
	"log"
	"strings"
)

func main() {
	//test := lib.GetTestInput(8)
	//run(test)
	input := lib.GetInput(2023, 8)
	run(input)
}

type node struct {
	left  string
	right string
}

func parseTree(input []string) map[string]node {
	nodes := make(map[string]node)
	for _, line := range input[2:] {
		line1 := strings.Split(strings.TrimSuffix(line, ")"), " = (")
		leaves := strings.Split(line1[1], ", ")
		nodes[line1[0]] = node{leaves[0], leaves[1]}
	}
	return nodes
}

func allEndingWithA(nodes map[string]node) []string {
	endingWithA := make([]string, 0)
	for k := range nodes {
		if k[2] == 'A' {
			endingWithA = append(endingWithA, k)
		}
	}
	return endingWithA
}

func greatestCommonDivisor(a, b int) int {
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

func lowestCommonMultiple(nums []int) int {
	if len(nums) < 2 {
		return nums[0]
	}
	result := nums[0] * nums[1] / greatestCommonDivisor(nums[0], nums[1])
	return lowestCommonMultiple(append([]int{result}, nums[2:]...))
}

func run(input []string) {
	instructions := strings.TrimSpace(input[0])

	nodes := parseTree(input)

	current := "AAA"
	counter := 0
	for {
		dir := instructions[counter%len(instructions)]
		switch dir {
		case 'L':
			current = nodes[current].left
		case 'R':
			current = nodes[current].right
		default:
			log.Fatalln("We messed up:", dir)
		}
		counter++
		if current == "ZZZ" {
			break
		}
	}
	log.Println("Part 1:", counter)

	currents := allEndingWithA(nodes)
	periods := make([]int, len(currents))
	for i := range currents {
		counter = 0
		for {
			dir := instructions[counter%len(instructions)]
			switch dir {
			case 'L':
				currents[i] = nodes[currents[i]].left
			case 'R':
				currents[i] = nodes[currents[i]].right
			default:
				log.Fatalln("We messed up:", dir)
			}
			counter++
			if currents[i][2] == 'Z' {
				periods[i] = counter
				break
			}
		}
	}
	log.Println("Part 2:", lowestCommonMultiple(periods))
}
