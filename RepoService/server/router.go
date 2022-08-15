package server

import (
	"RepoService/controllers"
	"net/http"

	"github.com/gorilla/mux"
)

var routeAuthRegistry = map[string][]string{
	"/source,GET":                     {"ALL"},
	"/source?owner=":                  {"ALL"},
	"/source/{id}":                    {"ALL"},
	"/source,POST":                    {"ROLE_USER"},
	"/source,DELETE":                  {"ROLE_USER, ROLE_ADMIN"},
	"/source/add-collabarator,DELETE": {"ROLE_USER"},
}

func SetupRouter() http.Handler {
	r := mux.NewRouter()

	InitMapper()

	//postoji query -> ?owner=example@gmail.com
	r.HandleFunc("/source", controllers.GetAllSourcesHandler).Methods(http.MethodGet)
	r.HandleFunc("/source/{id}", controllers.GetSourceByIdHandler).Methods(http.MethodGet)
	r.HandleFunc("/source", controllers.CreateSourceHandler).Methods(http.MethodPost)
	r.HandleFunc("/source/{id}", controllers.DeleteSourceByIdHandler).Methods(http.MethodDelete)
	r.HandleFunc("/source/add-collabarator", controllers.AddColabaratorHandler).Methods(http.MethodPost)
	// r.HandleFunc("/pull/{id}", controllers.DeleteRepo).Methods(http.MethodGet)
	// r.HandleFunc("/push", controllers.DeleteRepo).Methods(http.MethodPost)
	// r.HandleFunc("/clone/{id}", controllers.DeleteRepo).Methods(http.MethodGet)

	r.Use(logRoute)
	r.Use(setupRouteAsJson)
	r.Use(handleJwt)

	return r
}
