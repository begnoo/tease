package controllers

import (
	"UserService/di"
	"UserService/domain"
	"UserService/errors"
	"UserService/request"
	"UserService/utils"
	"encoding/json"
	"io"
	"net/http"

	"github.com/devfeel/mapper"
	"github.com/gorilla/mux"
)

func GetAllUsersHandler(w http.ResponseWriter, r *http.Request) {
	userService := di.InitializeUserService()
	data, err := userService.ReadAll()

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

func CreateUserHandler(w http.ResponseWriter, r *http.Request) {
	var requestBody request.CreateUserRequest
	json.NewDecoder(r.Body).Decode(&requestBody)
	r.Body.Close()

	err := ValidateStruct(requestBody)

	if !errors.HandleHttpError(err, w) {
		return
	}

	var user domain.User
	mapper.AutoMapper(&requestBody, &user)

	userService := di.InitializeUserService()
	res, err := userService.CreateUser(user, "ROLE_USER")

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(res)
	io.WriteString(w, result)
}

func VerifyUserExistsHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	email := vars["email"]
	if email == "" {
		return
	}

	userService := di.InitializeUserService()
	res := userService.VerifyUserExists(email)

	data := map[string]bool{
		"result": res,
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}
