package common

import (
	"bufio"
	"os"
)

type LineReader func() (string, bool, error)

func ReadFileByLine(filePath string) (LineReader, func(), error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, nil, err
	}

	scanner := bufio.NewScanner(file)

	nextLine := func() (string, bool, error) {
		if scanner.Scan() {
			return scanner.Text(), false, nil
		}

		// This checks for errors reading file
		if err := scanner.Err(); err != nil {
			return "", false, err
		}

		// Reached end of file
		return "", true, nil
	}

	cleanup := func() {
		file.Close()
	}

	return nextLine, cleanup, nil
}
