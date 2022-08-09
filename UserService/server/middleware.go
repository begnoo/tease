package server

import (
	"fmt"
	"net/http"
)

func logRoute(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// log uri
		fmt.Println(r.RequestURI)

		next.ServeHTTP(w, r)
	})
}

func setupRouteAsJson(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// make every response json
		w.Header().Set("Content-Type", "application/json")
		next.ServeHTTP(w, r)
	})
}

func handleJwt(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		//TODO: Uraditi jwt validaciju
		next.ServeHTTP(w, r)
	})
}
