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
	"github.com/golang-jwt/jwt/v4"
	"github.com/gorilla/mux"
)

func CreateCloneHandler(w http.ResponseWriter, r *http.Request) {
	var requestBody request.Commit
	json.NewDecoder(r.Body).Decode(&requestBody)
	r.Body.Close()

	err := ValidateStruct(requestBody)
	if !errors.HandleHttpError(err, w) {
		return
	}

	var commit domain.Commit
	mapper.AutoMapper(&requestBody, &commit)

	token, err := security.ParseTokenFromRequest(r)
	if !errors.HandleHttpError(err, w) {
		return
	}

	claims := token.Claims.(jwt.MapClaims)
	fmt.Printf("%+v\n", claims)

	commitService := di.InitializeCommitService()
	data, err := commitService.Create(commit)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

func CloneReadBySourceHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	owner := vars["owner"]
	name := vars["source"]

	cloneService := di.InitializeCommitService()
	data, err := cloneService.ReadBySource(owner, name)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}
