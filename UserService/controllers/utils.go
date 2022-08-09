package controllers

import (
	"encoding/json"
	"fmt"
)

func struct_to_json(data interface{}) string {
	result, err := json.Marshal(data)
	if err != nil {
		fmt.Println("Couldn't marshal provided data", data)
	}
	return string(result)
}
