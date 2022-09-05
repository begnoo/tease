package utils

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"net/http"
	"strings"
	"time"
)

func StructToJson(data interface{}) string {
	result, err := json.Marshal(data)
	if err != nil {
		fmt.Println("Couldn't marshal provided data", data)
	}
	return string(result)
}

type ErrorModel struct {
	Key     string
	Message string
}

func ParseValidationErrToJson(err_str string) []ErrorModel {
	errors := strings.Split(err_str, "\n")
	arr := []ErrorModel{}
	for _, err := range errors {
		parts := strings.Split(err, "Error")
		fullKey := strings.Trim(parts[0][4:], " ")
		key := strings.Split(fullKey[1:len(fullKey)-1], ".")[1]
		message := strings.Trim(parts[1][1:], " ")
		arr = append(arr, ErrorModel{Key: key, Message: message})
	}

	return arr
}

func Contains(s []interface{}, e interface{}) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}

var letterRunes = []rune("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")

func RandStringRunes(n int) string {
	rand.Seed(time.Now().UnixNano())
	b := make([]rune, n)
	for i := range b {
		b[i] = letterRunes[rand.Intn(len(letterRunes))]
	}
	return string(b)
}

func SendMail(owner, source, user string) {
	postBody, _ := json.Marshal(map[string]string{
		"source": source,
		"owner":  owner,
		"user":   user,
	})
	responseBody := bytes.NewBuffer(postBody)
	//Leverage Go's HTTP Post function to make request
	resp, err := http.Post("http://0.0.0.0:30000/collab", "application/json", responseBody)
	//Handle Error
	if err != nil {
		log.Fatalf("An Error Occured %v", err)
	}
	defer resp.Body.Close()
}

func SendAcceptMail(owner, source, user string) {
	postBody, _ := json.Marshal(map[string]string{
		"source": source,
		"owner":  owner,
		"user":   user,
	})
	responseBody := bytes.NewBuffer(postBody)
	//Leverage Go's HTTP Post function to make request
	resp, err := http.Post("http://0.0.0.0:30000/collab/accept", "application/json", responseBody)
	//Handle Error
	if err != nil {
		log.Fatalf("An Error Occured %v", err)
	}
	defer resp.Body.Close()
}

func SendRejectMail(owner, source, user string) {
	postBody, _ := json.Marshal(map[string]string{
		"source": source,
		"owner":  owner,
		"user":   user,
	})
	responseBody := bytes.NewBuffer(postBody)
	//Leverage Go's HTTP Post function to make request
	resp, err := http.Post("http://0.0.0.0:30000/collab/reject", "application/json", responseBody)
	//Handle Error
	if err != nil {
		log.Fatalf("An Error Occured %v", err)
	}
	defer resp.Body.Close()
}
