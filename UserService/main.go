package main

import (
	"UserService/server"
	"net/http"
)

func main() {
	r := server.SetupRouter()

	http.ListenAndServe(":8080", r)
}
