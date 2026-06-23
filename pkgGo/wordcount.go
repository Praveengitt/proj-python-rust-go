package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	// Map to store word counts (like Python dict / Java HashMap)
	wordCount := make(map[string]int)

	// Open file
	file, err := os.Open("C:\\pravin\\Projects\\pravinrepo\\python-rust-go\\words.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	// Read file line by line
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		// Split into words (like regex split in Java/Python)
		words := strings.Fields(line)

		for _, word := range words {
			word = strings.ToLower(word)
			wordCount[word]++
		}
	}

	// Check scanner errors
	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	// Print result
	fmt.Println(wordCount)
}