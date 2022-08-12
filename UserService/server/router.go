package server

import (
	"net/http"

	"UserService/controllers"

	"github.com/gorilla/mux"
)

var routeAuthRegistry map[string]string

func setupRouteAuthorization() {
	routeAuthRegistry = map[string]string{
		"/users,GET":       "ROLE_USER",
		"/users,POST":      "ALL",
		"/auth/login,POST": "ALL",
	}
}

func SetupRouter() http.Handler {
	r := mux.NewRouter()

	r.HandleFunc("/users", controllers.GetAllUsersHandler).Methods(http.MethodGet)
	r.HandleFunc("/users", controllers.CreateUserHandler).Methods(http.MethodPost)
	r.HandleFunc("/auth/login", controllers.Login).Methods(http.MethodPost)

	setupRouteAuthorization()

	r.Use(logRoute)
	r.Use(setupRouteAsJson)
	r.Use(handleJwt)

	return r
}
