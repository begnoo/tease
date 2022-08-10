package controllers

import (
	"UserService/domain"
	"UserService/service"
	"encoding/json"
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

func CreateUserHandler(w http.ResponseWriter, r *http.Request) {

	var user domain.User
	json.NewDecoder(r.Body).Decode(&user)

	userService := service.NewUserService()
	res, err := userService.CreateUser(user)

	if err != nil {
		w.WriteHeader(http.StatusOK)
		io.WriteString(w, fmt.Sprintf("Error: %s", err))
		return
	}

	w.WriteHeader(http.StatusOK)
	result := struct_to_json(res)
	io.WriteString(w, result)
}
