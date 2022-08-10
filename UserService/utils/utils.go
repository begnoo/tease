package utils

import (
	"encoding/json"
	"fmt"
	"strings"

	"github.com/go-playground/validator/v10"
)

var validate *validator.Validate = nil

func StructToJson(data interface{}) string {
	result, err := json.Marshal(data)
	if err != nil {
		fmt.Println("Couldn't marshal provided data", data)
	}
	return string(result)
}

func initValidator() *validator.Validate {
	if validate == nil {
		validate = validator.New()
	}

	return validate
}

func ValidateStruct(data interface{}) (bool, string) {
	val := initValidator()
	err := val.Struct(data)
	if err != nil {
		return false, StructToJson(parseErrToJson(err.Error()))
	}

	return true, ""
}

type ErrorModel struct {
	Key     string
	Message string
}

func parseErrToJson(err_str string) []ErrorModel {
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
