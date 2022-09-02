package server

import (
	"StatsService/controllers"
	"net/http"

	"github.com/gorilla/mux"
)

var authReg = map[string][]string{
	"/commits,POST":                         {"ROLE_USER"},
	"/commits/multi,POST":                   {"ROLE_USER"},
	"/commits/{user}/by-user,GET":           {"ALL"},
	"/commits/{owner}/{source},GET":         {"ALL"},
	"/commits/{owner}/{source}/by-user,GET": {"ALL"},
	"/clones/{owner}/{source},GET":          {"ALL"},
	"/clones,POST":                          {"ROLE_USER"},
}

func SetupRouter() http.Handler {
	r := mux.NewRouter()

	InitMapper()

	r.HandleFunc("/commits",
		handleJwt(controllers.CreateCommitHandler, authReg["/commits,POST"])).
		Methods(http.MethodPost)
	r.HandleFunc("/commits/multi",
		handleJwt(controllers.CreateCommitsHandler, authReg["/commits/multi,POST"])).
		Methods(http.MethodPost)
	r.HandleFunc("/commits/{owner}/{source}",
		handleJwt(controllers.ReadBySourceHandler, authReg["/commits/{owner}/{source},GET"])).
		Methods(http.MethodGet)
	r.HandleFunc("/commits-by-user/{user}",
		handleJwt(controllers.ReadByUserHandler, authReg["/commits/{user}/by-user,GET"])).
		Methods(http.MethodGet)
	r.HandleFunc("/commits/{owner}/{source}/by-user",
		handleJwt(controllers.ReadBySourceGroupByUserHandler, authReg["/commits/{owner}/{source}/by-user,GET"])).
		Methods(http.MethodGet)

	r.HandleFunc("/clones/{owner}/{source}",
		handleJwt(controllers.CloneReadBySourceHandler, authReg["/clones/{owner}/{source},GET"])).
		Methods(http.MethodGet)
	r.HandleFunc("/clones/{owner}/{source}",
		handleJwt(controllers.CreateCloneHandler, authReg["/clones,POST"])).
		Methods(http.MethodGet)

	r.Use(logRoute)
	r.Use(setupRouteAsJson)

	return r
}
