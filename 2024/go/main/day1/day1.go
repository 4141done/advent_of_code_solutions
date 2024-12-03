// task: using the input, pair the values so that the lowest numbers are paired,
// then the next lowest, and so on. Then, calculate the difference between those numbers and add them all up

package day1

import (
	"fmt"
	"sort"

	"main/common"
)

func Run() {
	filePath := "day1/input.txt"

	nextLine, cleanup, err := common.ReadFileByLine(filePath)
	if err != nil {
		fmt.Printf("error opening file: %v\n", err)
		return
	}

	defer cleanup()

	var left []int
	var right []int
	var result int = 0

	for {
		line, isEndOfFile, err := nextLine()
		if err != nil {
			fmt.Printf("error reading line: %v\n", err)
			return
		}

		if isEndOfFile {
			break
		}

		num1, num2, err := parseLine(line)

		if err != nil {
			fmt.Printf("error parsing line %v\n", line)
		}

		left = append(left, num1)
		right = append(right, num2)
	}

	sort.Ints(left)
	sort.Ints(right)

	for i, leftItem := range left {
		rightItem := right[i]

		result += getDistance(leftItem, rightItem)
	}

	fmt.Printf("The answer to part 1 is: %d\n", result)
}

func Run2() {
	filePath := "day1/input.txt"

	nextLine, cleanup, err := common.ReadFileByLine(filePath)
	if err != nil {
		fmt.Printf("error opening file: %v\n", err)
		return
	}

	defer cleanup()

	var left []int
	var right = make(map[int]int)
	var result int = 0

	for {
		line, isEndOfFile, err := nextLine()
		if err != nil {
			fmt.Printf("error reading line: %v\n", err)
			return
		}

		if isEndOfFile {
			break
		}

		num1, num2, err := parseLine(line)

		if err != nil {
			fmt.Printf("error parsing line %v\n", line)
		}

		left = append(left, num1)
		right[num2] = right[num2] + 1
	}

	for _, num := range left {
		result += num * right[num]
	}

	fmt.Printf("The answer to part 2 is: %d", result)
}

func parseLine(line string) (int, int, error) {
	var num1 int
	var num2 int
	_, err := fmt.Sscanf(line, "%d %d", &num1, &num2)

	if err != nil {
		return 0, 0, err
	}

	return num1, num2, nil
}

// Forced to do this garbage because Go forces you to play type games just to get
// absolute value of an int
func getDistance(num1 int, num2 int) int {
	if num1 > num2 {
		return num1 - num2
	}

	return num2 - num1
}
