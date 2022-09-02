package controllers

import (
	"StatsService/di"
	"StatsService/domain"
	"StatsService/errors"
	"StatsService/request"
	"StatsService/security"
	"StatsService/utils"
	"encoding/json"
	"fmt"
	"io"
	"net/http"

	"github.com/devfeel/mapper"
	"github.com/gorilla/mux"
)

func CreateCommitHandler(w http.ResponseWriter, r *http.Request) {
	var requestBody request.Commit
	json.NewDecoder(r.Body).Decode(&requestBody)
	r.Body.Close()

	err := ValidateStruct(requestBody)
	if !errors.HandleHttpError(err, w) {
		return
	}

	var commit domain.Commit
	mapper.AutoMapper(&requestBody, &commit)

	_, err = security.ParseTokenFromRequest(r)
	if !errors.HandleHttpError(err, w) {
		return
	}

	commitService := di.InitializeCommitService()
	data, err := commitService.Create(commit)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

func ReadBySourceHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	owner := vars["owner"]
	name := vars["source"]

	commitService := di.InitializeCommitService()
	data, err := commitService.ReadBySource(owner, name)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

func ReadByUserHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	user := vars["user"]
	fmt.Printf("user: %s\n", user)

	commitService := di.InitializeCommitService()
	data, err := commitService.ReadByUser(user)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

func ReadBySourceGroupByUserHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	owner := vars["owner"]
	name := vars["source"]

	commitService := di.InitializeCommitService()
	data, err := commitService.ReadBySourceGroupByUser(owner, name)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}
