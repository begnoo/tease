package main

import (
	"UserService/server"
	"fmt"
	"log"
	"net/http"

	"github.com/joho/godotenv"
)

// TODO: Dodaj JWT							**
// TODO: Api Gateway						***
func main() {

	err := godotenv.Load(".env")

	if err != nil {
		log.Fatal("Error loading .env file")
	}

	r := server.SetupRouter()

	err = http.ListenAndServe(":8080", r)

	if err == nil {
		fmt.Printf("Started listening on port 8080")
	}
}
