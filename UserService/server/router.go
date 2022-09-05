package server

import (
	"net/http"

	"UserService/controllers"

	"github.com/gorilla/mux"
)

var authReg = map[string][]string{
	"/users,GET":                    {"ROLE_USER", "ROLE_ADMIN"},
	"/users,POST":                   {"ALL"},
	"/auth/login,POST":              {"ALL"},
	"/users/verify-user-exists,GET": {"ALL"},
}

func SetupRouter() http.Handler {
	r := mux.NewRouter()

	InitMapper()

	r.HandleFunc("/users",
		handleJwt(controllers.GetAllUsersHandler, authReg["/users,GET"])).Methods(http.MethodGet)
	r.HandleFunc("/users",
		controllers.CreateUserHandler).Methods(http.MethodPost)
	r.HandleFunc("/auth/login",
		controllers.Login).Methods(http.MethodPost)
	r.HandleFunc("/users/verify-user-exists/{email}",
		controllers.VerifyUserExistsHandler).Methods(http.MethodGet)
	r.HandleFunc("/users/search/{keyword}",
		controllers.SearchUsersHandler).Methods(http.MethodGet)

	r.Use(logRoute)
	r.Use(setupRouteAsJson)

	return r
}
