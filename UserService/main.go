package main

import (
	"UserService/server"
	"fmt"
	"net/http"
)

func main() {
	r := server.SetupRouter()

	err := http.ListenAndServe(":8080", r)

	if err == nil {
		fmt.Printf("Started listening on port 8080")
	}
}
