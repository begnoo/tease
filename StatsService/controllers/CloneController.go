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
	var requestBody request.Clone
	json.NewDecoder(r.Body).Decode(&requestBody)
	r.Body.Close()

	err := ValidateStruct(requestBody)
	if !errors.HandleHttpError(err, w) {
		return
	}

	var clone domain.Clone
	mapper.AutoMapper(&requestBody, &clone)

	token, err := security.ParseTokenFromRequest(r)
	if !errors.HandleHttpError(err, w) {
		return
	}

	claims := token.Claims.(jwt.MapClaims)
	fmt.Printf("%+v\n", claims)

	cloneService := di.InitializeCloneService()
	data, err := cloneService.Create(clone)

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

	cloneService := di.InitializeCloneService()
	data, err := cloneService.ReadBySource(owner, name)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}
