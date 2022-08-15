package server

import (
	"RepoService/security"
	"fmt"
	"net/http"

	"github.com/golang-jwt/jwt/v4"
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

func handleJwt(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

		uri := fmt.Sprintf("%s,%s", r.RequestURI, r.Method)

		if checkRouteAuth(uri, "ALL") {
			next.ServeHTTP(w, r)
			return
		}

		token, err := security.ParseTokenFromRequest(r)

		if err != nil {
			w.WriteHeader(http.StatusUnauthorized)
			return
		}

		tokenRole := token.Claims.(jwt.MapClaims)["role"].(string)

		if checkRouteAuth(uri, tokenRole) {
			next.ServeHTTP(w, r)
			return
		}

		w.WriteHeader(http.StatusUnauthorized)
	})
}

func checkRouteAuth(url string, role string) bool {
	if authRoles, ok := routeAuthRegistry[url]; ok {
		if utils.Contains(authRoles, role) {
			return true
		} else {
			return false
		}
	}
	return true
}
