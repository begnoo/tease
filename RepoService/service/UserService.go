package service

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
)

type Result struct {
	Result bool
}

func VerifyUserExists(userEmail string) bool {
	resp, err := http.Get(fmt.Sprintf("%s/verify-user-exists/%s", os.Getenv("USER_SERVICE_URL"), userEmail))
	if err != nil {
		return false
	}
	defer resp.Body.Close()

	var value Result
	json.NewDecoder(resp.Body).Decode(&value)
	return value.Result
}
