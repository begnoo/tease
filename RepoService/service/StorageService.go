package service

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
)

func CreateSourceInStorage(userEmail string, name string) bool {
	resp, err := http.Get(fmt.Sprintf("%s/init/%s/%s", os.Getenv("STORAGE_SERVICE_URL"), userEmail, name))
	if err != nil {
		return false
	}
	defer resp.Body.Close()

	var value Result
	json.NewDecoder(resp.Body).Decode(&value)
	return value.Result
}
