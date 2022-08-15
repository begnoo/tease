package controllers

import (
	"RepoService/di"
	"RepoService/errors"
	"RepoService/utils"
	"io"
	"net/http"
	"strconv"

	"github.com/gorilla/mux"
)

func AcceptInviteHandler(w http.ResponseWriter, r *http.Request) {
	collabService := di.InitializeCollabService()
	vars := mux.Vars(r)
	id_string := vars["id"]

	id, err := strconv.Atoi(id_string)
	if !errors.HandleHttpError(err, w) {
		return
	}

	data, err := collabService.AcceptInvite(id)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

func RejectInviteHandler(w http.ResponseWriter, r *http.Request) {
	collabService := di.InitializeCollabService()
	vars := mux.Vars(r)
	id_string := vars["id"]

	id, err := strconv.Atoi(id_string)
	if !errors.HandleHttpError(err, w) {
		return
	}

	data, err := collabService.RejectInvite(id)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}

func DeleteCollabaratorHandler(w http.ResponseWriter, r *http.Request) {
	collabService := di.InitializeCollabService()
	vars := mux.Vars(r)
	id_string := vars["id"]

	id, err := strconv.Atoi(id_string)
	if !errors.HandleHttpError(err, w) {
		return
	}

	data, err := collabService.DeleteCollabarator(id)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(data)
	io.WriteString(w, result)
}
