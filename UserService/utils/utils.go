package utils

import (
	"encoding/json"
	"fmt"
	"strings"
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
