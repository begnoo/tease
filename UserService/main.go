package main

import (
	"UserService/server"
	"fmt"
	"log"
	"net/http"

	"github.com/joho/godotenv"
)

// TODO: Api Gateway						***
func main() {

	err := godotenv.Load(".env")

	if err != nil {
		log.Fatal("Error loading .env file")
	}

	r := server.SetupRouter()
	handler := server.SetupCors(&r)
	err = http.ListenAndServe(":8080", handler)

	if err == nil {
		fmt.Printf("Started listening on port 8080")
	}
}
