package controllers

import (
	"UserService/di"
	"UserService/domain"
	"UserService/request"
	"UserService/utils"
	"encoding/json"
	"fmt"
	"io"
	"net/http"

	"github.com/devfeel/mapper"
	"github.com/gorilla/mux"
)

func GetUser(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)

	vars := mux.Vars(r)
	id := vars["id"]

	result := utils.StructToJson(id)
	io.WriteString(w, result)
}

func GetAllUsersHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	userService := di.InitializeUserService()

	data, err := userService.ReadAll()

	if err != nil {
		io.WriteString(w, fmt.Sprintf("Error: %s", err))
	}

	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

func CreateUserHandler(w http.ResponseWriter, r *http.Request) {

	var requestBody request.CreateUserRequest
	json.NewDecoder(r.Body).Decode(&requestBody)
	valid, errors := utils.ValidateStruct(requestBody)
	if !valid {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, errors)
		return
	}

	var user domain.User
	mapper.AutoMapper(&requestBody, &user)

	userService := di.InitializeUserService()
	res, err := userService.CreateUser(user)

	if err != nil {
		w.WriteHeader(http.StatusOK)
		io.WriteString(w, fmt.Sprintf("Error: %s", err))
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(res)
	io.WriteString(w, result)
}
