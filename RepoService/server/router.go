package server

import (
	"RepoService/controllers"
	"net/http"

	"github.com/gorilla/mux"
)

var authReg = map[string][]string{
	"/source,GET":                          {"ALL"},
	"/source?owner=":                       {"ALL"},
	"/source/{id}":                         {"ALL"},
	"/source,POST":                         {"ROLE_USER"},
	"/source,DELETE":                       {"ROLE_USER", "ROLE_ADMIN"},
	"/source/search/{keyword},GET":         {"ALL"},
	"/source/access,POST":                  {"ROLE_USER"},
	"/collabs/add,POST":                    {"ROLE_USER"},
	"/collabs/{id}/accept,POST":            {"ROLE_USER"},
	"/collabs/{id}/reject,POST":            {"ROLE_USER"},
	"/collabs/{id}/delete,DELETE":          {"ROLE_USER"},
	"/collabs/{name}/by-name,GET":          {"ROLE_USER"},
	"/collabs/{owner}/source/{source},GET": {"ALL"},
}

func SetupRouter() http.Handler {
	r := mux.NewRouter()

	InitMapper()

	// Sources
	r.HandleFunc("/source", controllers.GetAllSourcesHandler).
		Methods(http.MethodGet)
	r.HandleFunc("/source/{id}", controllers.GetSourceByIdHandler).
		Methods(http.MethodGet)
	r.HandleFunc("/source",
		handleJwt(controllers.CreateSourceHandler, authReg["/source,POST"])).Methods(http.MethodPost)
	r.HandleFunc("/source/{id}",
		handleJwt(controllers.DeleteSourceByIdHandler, authReg["/source,DELETE"])).Methods(http.MethodDelete)
	r.HandleFunc("/source/search/{keyword}",
		handleJwt(controllers.SearchSourcesHandler, authReg["/source/search/{keyword},GET"])).Methods(http.MethodGet)
	r.HandleFunc("/source/access",
		handleJwt(controllers.HasAccessHandler, authReg["/source/access,POST"])).Methods(http.MethodPost)

	// Collabs
	r.HandleFunc("/collabs/add",
		handleJwt(controllers.AddColabaratorHandler, authReg["/collabs/add,POST"])).Methods(http.MethodPost)
	r.HandleFunc("/collabs/{id}/accept",
		handleJwt(controllers.AcceptInviteHandler, authReg["/collabs/{id}/accept,POST"])).Methods(http.MethodPost)
	r.HandleFunc("/collabs/{id}/reject",
		handleJwt(controllers.RejectInviteHandler, authReg["/collabs/{id}/reject,POST"])).Methods(http.MethodPost)
	r.HandleFunc("/collabs/{id}",
		handleJwt(controllers.DeleteCollabaratorHandler, authReg["/collabs/{id}/delete,DELETE"])).Methods(http.MethodDelete)
	r.HandleFunc("/collabs/{owner}/source/{source}",
		handleJwt(controllers.GetCollabaratorsHandler, authReg["/collabs/{owner}/source/{source},GET"])).Methods(http.MethodGet)
	r.HandleFunc("/collabs/{name}/by-name",
		handleJwt(controllers.GetCollabaratorsByNameHandler, authReg["/collabs/{name}/by-name,GET"])).Methods(http.MethodGet)

	r.Use(logRoute)
	r.Use(setupRouteAsJson)

	return r
}
