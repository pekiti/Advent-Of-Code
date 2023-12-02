package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var inputLines = readlines()

func readlines() []string {
	f, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	var lines []string
	s := bufio.NewScanner(f)
	for s.Scan() {
		lines = append(lines, s.Text())
	}
	return lines
}

func part1() int {
	sum := 0
	for _, line := range inputLines {
		first := -1
		last := 0
		for i := 0; i < len(line); i++ {
			if line[i] >= '0' && line[i] <= '9' {
				if first == -1 {
					first = int(line[i] - '0')
				}
				last = int(line[i] - '0')
			}
		}
		total := first*10 + last
		sum += total
	}
	return sum
}

func part2() int {

	substrings := map[string]int{
		"one":   1,
		"two":   2,
		"three": 3,
		"four":  4,
		"five":  5,
		"six":   6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
		"1":     1,
		"2":     2,
		"3":     3,
		"4":     4,
		"5":     5,
		"6":     6,
		"7":     7,
		"8":     8,
		"9":     9,
	}

	sum := 0
	for _, line := range inputLines {
		first := -1
		last := 0
		valleft, valright := findKeywordsInString(line, &substrings)
		if valleft != -1 && valright != -1 {
			if first == -1 {
				first = valleft
			}
			last = valright
		}
		total := first*10 + last
		sum += total
	}
	return sum
}

func findKeywordsInString(str string, keywords *map[string]int) (int, int) {
	if len(str) == 0 {
		return -1, -1
	}
	leftidx := 1000
	rightidx := -1000
	valleft := -1
	valright := -1
	for keyword, v := range *keywords {
		idx := strings.Index(str, keyword)
		if idx != -1 && idx < leftidx {
			leftidx = idx
			valleft = v
		}
		idx = strings.LastIndex(str, keyword)
		if idx != -1 && idx > rightidx {
			rightidx = idx
			valright = v
		}
	}
	return valleft, valright
}

func main() {
	fmt.Println("Advent of Code 2023 - Day 1")
	fmt.Println("Solution - Part 1:", part1())
	fmt.Println("Solution - Part 2:", part2())
}
