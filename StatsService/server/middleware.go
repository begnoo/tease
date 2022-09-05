package server

import (
	"StatsService/security"
	"fmt"
	"net/http"

	"github.com/golang-jwt/jwt/v4"
	"github.com/rs/cors"
	"gorm.io/gorm/utils"
)

func logRoute(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// log uri
		fmt.Printf("%s,%s\n", r.RequestURI, r.Method)

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

func handleJwt(f func(w http.ResponseWriter, r *http.Request), roles []string) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {

		if utils.Contains(roles, "ALL") {
			f(w, r)
			return
		}

		token, err := security.ParseTokenFromRequest(r)

		if err != nil {
			w.WriteHeader(http.StatusUnauthorized)
			return
		}

		tokenRole := token.Claims.(jwt.MapClaims)["role"].(string)

		if utils.Contains(roles, tokenRole) {
			f(w, r)
			return
		}

		w.WriteHeader(http.StatusUnauthorized)
	}
}

func SetupCors(routes *http.Handler) http.Handler {
	corsHandler := cors.New(cors.Options{
		AllowedOrigins: []string{"*"},
		AllowedMethods: []string{http.MethodPost, http.MethodConnect, http.MethodGet, http.MethodDelete,
			http.MethodHead, http.MethodOptions, http.MethodPut, http.MethodTrace},
		AllowedHeaders:   []string{"*"},
		AllowCredentials: false,
	})

	return corsHandler.Handler(*routes)
}
