package main

import (
	"StatsService/server"
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/joho/godotenv"
)

func main() {

	err := godotenv.Load(".env")

	if err != nil {
		log.Fatal("Error loading .env file")
	}

	r := server.SetupRouter()
	handler := server.SetupCors(&r)
	err = http.ListenAndServe(os.Getenv("SERVER_PORT"), handler)
	if err != nil {
		fmt.Printf("Couldn't start listening on port %s\n", os.Getenv("SERVER_PORT"))
	}
}
