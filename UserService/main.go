package main

import (
	"UserService/server"
	"fmt"
	"net/http"
)

// TODO: Pogledaj da li mozes validaciju da gurnes u middleware ili elegantnije da odradis...
// TODO: Error handling za http handlere	*
// TODO: Dodaj JWT							**
// TODO: Api Gateway						***
func main() {
	r := server.SetupRouter()

	err := http.ListenAndServe(":8080", r)

	if err == nil {
		fmt.Printf("Started listening on port 8080")
	}
}
