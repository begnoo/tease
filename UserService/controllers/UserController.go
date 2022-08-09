package controllers

import (
	"UserService/domain"
	"fmt"
	"io"
	"net/http"

	"github.com/gorilla/mux"
)

func GetUser(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)

	vars := mux.Vars(r)
	id := vars["id"]

	result := struct_to_json(id)
	fmt.Print(result)
	io.WriteString(w, result)
}

func GetAllUsersHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)

	user := domain.User{
		Username: "ovca",
		Email:    "foca@goca.com",
		Password: "blabla",
	}

	result := struct_to_json(user)
	fmt.Print(result)
	io.WriteString(w, result)
}
