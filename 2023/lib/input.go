package lib

import (
	"io"
	"log"
	"net/http"
	"os"
	"strconv"
	"strings"
)

// Returns lines as a string array
func GetInput(year int, day int) []string {
	cacheFile := "input/" + strconv.Itoa(day) + ".txt"
	bytes, err := os.ReadFile(cacheFile)
	if err == nil {
		return strings.Split(strings.TrimSpace(string(bytes)), "\n")
	}
	bytes, err = os.ReadFile("/home/keval/.aoc_cookie")
	if err != nil {
		log.Fatalln("Failed to read ~/.aoc_cookie: " + err.Error())
	}
	cookie := http.Cookie{Name: "session", Value: strings.TrimSpace(string(bytes))}
	url := "https://adventofcode.com/" + strconv.Itoa(year) + "/day/" + strconv.Itoa(day) + "/input"
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Fatalln("Failed to create input download request: " + err.Error())
	}
	req.Header.Set("COOKIE", "session=")
	req.AddCookie(&cookie)
	client := http.Client{}
	response, err := client.Do(req)
	if err != nil {
		log.Fatalln("Failed to download input: ", err.Error())
	}
	bytes, err = io.ReadAll(response.Body)
	if err != nil {
		log.Fatalln("Failed to read response body: " + err.Error())
	}
	err = os.WriteFile(cacheFile, bytes, 0666)
	if err != nil {
		log.Println("Warning, failed to cache input: ", err.Error())
	}
	return strings.Split(strings.TrimSpace(string(bytes)), "\n")
}
