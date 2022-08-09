package server

import (
	"net/http"

	"UserService/controllers"

	"github.com/gorilla/mux"
)

func SetupRouter() http.Handler {
	r := mux.NewRouter()

	r.HandleFunc("/users", controllers.GetAllUsersHandler).Methods(http.MethodGet)
	r.HandleFunc("/users/{id}", controllers.GetUser).Methods(http.MethodGet)

	r.Use(logRoute)
	r.Use(setupRouteAsJson)

	return r
}
