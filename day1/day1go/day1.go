package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

func main() {
	a()
	b()
}

func a() {
	elfCals := input_to_slice()
	fmt.Printf("Most calories: %d\n", elfCals[0])
}

func b() {
	elfCals := input_to_slice()

	topCals := 0
	for _, cals := range elfCals[0:3] {
		topCals += cals
	}
	fmt.Printf("top 3: %d calories", topCals)
}

func input_to_slice() []int {
	file, err := os.Open("../input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	elfCals := []int{}
	cals := 0
	for scanner.Scan() {
		val := scanner.Text()
		if val == "" {
			elfCals = append(elfCals, cals)
			cals = 0
			continue
		}
		parsedCals, err := strconv.Atoi(val)
		if err != nil {
			log.Fatal(err)
		}
		cals += parsedCals
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	sort.Slice(elfCals, func(i, j int) bool {
		return elfCals[i] > elfCals[j]
	})
	return elfCals
}
