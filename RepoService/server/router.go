package server

import (
	"RepoService/controllers"
	"net/http"

	"github.com/gorilla/mux"
)

var authReg = map[string][]string{
	"/source,GET":                        {"ALL"},
	"/source?owner=":                     {"ALL"},
	"/source/{id}":                       {"ALL"},
	"/source,POST":                       {"ROLE_USER"},
	"/source,DELETE":                     {"ROLE_USER, ROLE_ADMIN"},
	"/source/collabs/add,POST":           {"ROLE_USER"},
	"/source/collabs/{id}/accept,GET":    {"ROLE_USER"},
	"/source/collabs/{id}/reject,GET":    {"ROLE_USER"},
	"/source/collabs/{id}/delete,DELETE": {"ROLE_USER"},
}

func SetupRouter() http.Handler {
	r := mux.NewRouter()

	InitMapper()

	r.HandleFunc("/source", controllers.GetAllSourcesHandler).
		Methods(http.MethodGet)
	r.HandleFunc("/source/{id}", controllers.GetSourceByIdHandler).
		Methods(http.MethodGet)
	r.HandleFunc("/source",
		handleJwt(controllers.CreateSourceHandler, authReg["/source,POST"])).Methods(http.MethodPost)
	r.HandleFunc("/source/{id}",
		handleJwt(controllers.DeleteSourceByIdHandler, authReg["/source,DELETE"])).Methods(http.MethodDelete)
	r.HandleFunc("/source/collabs/add",
		handleJwt(controllers.AddColabaratorHandler, authReg["/source/collabs/add,POST"])).Methods(http.MethodPost)
	r.HandleFunc("/source/collabs/{id}/accept",
		handleJwt(controllers.AcceptInviteHandler, authReg["/source/collabs/{id}/accept,GET"])).Methods(http.MethodGet)
	r.HandleFunc("/source/collabs/{id}/reject",
		handleJwt(controllers.RejectInviteHandler, authReg["/source/collabs/{id}/reject,GET"])).Methods(http.MethodGet)
	r.HandleFunc("/source/collabs/{id}/delete",
		handleJwt(controllers.DeleteCollabaratorHandler, authReg["/source/collabs/{id}/delete,DELETE"])).Methods(http.MethodDelete)
	// r.HandleFunc("/pull/{id}", controllers.DeleteRepo).Methods(http.MethodGet)
	// r.HandleFunc("/push", controllers.DeleteRepo).Methods(http.MethodPost)
	// r.HandleFunc("/clone/{id}", controllers.DeleteRepo).Methods(http.MethodGet)

	r.Use(logRoute)
	r.Use(setupRouteAsJson)

	return r
}
