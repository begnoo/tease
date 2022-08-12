package server

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"strings"

	"github.com/golang-jwt/jwt/v4"
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

		token, err := parseTokenFromRequest(r)

		if err != nil {
			w.WriteHeader(http.StatusUnauthorized)
			reserr := fmt.Errorf("nothing")
			json.NewEncoder(w).Encode(reserr)
			return
		}

		tokenRole := token.Claims.(jwt.MapClaims)["role"]

		if checkRouteAuth(uri, tokenRole) {
			next.ServeHTTP(w, r)
			return
		}

		w.WriteHeader(http.StatusUnauthorized)
		reserr := fmt.Errorf("not authorized")
		json.NewEncoder(w).Encode(reserr)
	})
}

func parseTokenFromRequest(r *http.Request) (*jwt.Token, error) {
	if r.Header["Authorization"] == nil {
		err := fmt.Errorf("no authorization header found")
		return nil, err
	}

	authHeader := r.Header["Authorization"][0]
	bearer := strings.Split(authHeader, " ")

	secretKey := os.Getenv("SECRET_KEY")
	var mySigningKey = []byte(secretKey)

	token, err := jwt.Parse(bearer[1], func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("there was an error in parsing")
		}
		return mySigningKey, nil
	})

	if err != nil {
		err := fmt.Errorf("your token has expired")
		return nil, err
	}

	return token, err
}

func checkRouteAuth(url string, role interface{}) bool {
	if authRole, ok := routeAuthRegistry[url]; ok {
		if authRole == "ALL" || authRole == role {
			return true
		} else {
			return false
		}
	}
	return true
}
