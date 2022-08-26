package controllers

import (
	"RepoService/di"
	"RepoService/domain"
	"RepoService/errors"
	"RepoService/request"
	"RepoService/responses"
	"RepoService/security"
	"RepoService/utils"
	"encoding/json"
	"io"
	"net/http"
	"strconv"

	"github.com/devfeel/mapper"
	"github.com/golang-jwt/jwt/v4"
	"github.com/gorilla/mux"
)

func GetAllSourcesHandler(w http.ResponseWriter, r *http.Request) {
	sourceService := di.InitializeSourceService()
	owner_query := r.URL.Query().Get("owner")

	var data *[]domain.Source
	var err error
	if owner_query != "" {
		data, err = sourceService.ReadByOwner(owner_query)
	} else {
		data, err = sourceService.ReadAll()
	}

	if !errors.HandleHttpError(err, w) {
		return
	}

	var sources []responses.Source
	for _, s := range *data {
		var source responses.Source
		mapper.Mapper(&s, &source)
		sources = append(sources, source)
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(sources)
	io.WriteString(w, result)
}

func CreateSourceHandler(w http.ResponseWriter, r *http.Request) {
	var requestBody request.CreateSourceRequest
	json.NewDecoder(r.Body).Decode(&requestBody)
	r.Body.Close()

	err := ValidateStruct(requestBody)
	if !errors.HandleHttpError(err, w) {
		return
	}

	var source domain.Source
	mapper.AutoMapper(&requestBody, &source)

	token, err := security.ParseTokenFromRequest(r)
	if !errors.HandleHttpError(err, w) {
		return
	}

	email := token.Claims.(jwt.MapClaims)["email"].(string)

	sourceService := di.InitializeSourceService()
	data, err := sourceService.Create(source, email)

	if !errors.HandleHttpError(err, w) {
		return
	}

	// var sourceResp responses.Source
	// mapper.AutoMapper(data, &sourceResp)

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

func GetSourcesByOwnerHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	owner := vars["owner"]

	sourceService := di.InitializeSourceService()
	data, err := sourceService.ReadByOwner(owner)

	if !errors.HandleHttpError(err, w) {
		return
	}

	var sources []responses.Source
	for _, s := range *data {
		var source responses.Source
		mapper.Mapper(&s, &source)
		sources = append(sources, source)
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(sources)
	io.WriteString(w, result)
}

func GetSourceByIdHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	id_string := vars["id"]
	id, err := strconv.Atoi(id_string)

	if !errors.HandleHttpError(err, w) {
		return
	}

	sourceService := di.InitializeSourceService()
	data, err := sourceService.ReadById(id)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

func DeleteSourceByIdHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	id_string := vars["id"]
	id, err := strconv.Atoi(id_string)

	if !errors.HandleHttpError(err, w) {
		return
	}

	token, err := security.ParseTokenFromRequest(r)
	if !errors.HandleHttpError(err, w) {
		return
	}

	email := token.Claims.(jwt.MapClaims)["email"].(string)

	sourceService := di.InitializeSourceService()
	data, err := sourceService.Delete(id, email)
	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

func AddColabaratorHandler(w http.ResponseWriter, r *http.Request) {
	var requestBody request.AddCollabaratorRequest
	json.NewDecoder(r.Body).Decode(&requestBody)
	r.Body.Close()

	token, err := security.ParseTokenFromRequest(r)
	if !errors.HandleHttpError(err, w) {
		return
	}

	owner := token.Claims.(jwt.MapClaims)["email"].(string)

	sourceService := di.InitializeSourceService()
	data, err := sourceService.AddColabarator(requestBody.Source, requestBody.Collabarator, owner)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

type Result struct {
	Result bool `json:"result"`
}

func HasAccessHandler(w http.ResponseWriter, r *http.Request) {
	var requestBody request.HasAccessRequest
	json.NewDecoder(r.Body).Decode(&requestBody)
	r.Body.Close()

	err := ValidateStruct(requestBody)
	if !errors.HandleHttpError(err, w) {
		return
	}

	sourceService := di.InitializeSourceService()
	res, err := sourceService.CollabaratorHasAccess(requestBody.User, requestBody.Owner, requestBody.SourceName)
	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(Result{Result: res})
	io.WriteString(w, result)
}
