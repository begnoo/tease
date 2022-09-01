package controllers

import (
	"RepoService/di"
	"RepoService/errors"
	"RepoService/responses"
	"RepoService/utils"
	"io"
	"net/http"
	"strconv"

	"github.com/devfeel/mapper"
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

	var resp_collab responses.Collabarator
	mapper.Mapper(data, &resp_collab)

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(resp_collab)
	io.WriteString(w, result)
}

func GetCollabaratorsHandler(w http.ResponseWriter, r *http.Request) {
	sourceService := di.InitializeSourceService()
	vars := mux.Vars(r)
	name := vars["source"]
	owner := vars["owner"]

	data, err := sourceService.GetCollabarators(owner, name)

	if !errors.HandleHttpError(err, w) {
		return
	}

	var res []responses.Collabarator
	for _, collab := range *data {
		var resp_collab responses.Collabarator
		mapper.Mapper(&collab, &resp_collab)
		res = append(res, resp_collab)
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(res)
	io.WriteString(w, result)
}

func GetCollabaratorsByNameHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	name := vars["name"]

	collabService := di.InitializeCollabService()
	data, err := collabService.ReadByName(name)
	println("ovo je name: %s", name)

	if !errors.HandleHttpError(err, w) {
		return
	}

	var res []responses.Collabarator
	for _, collab := range *data {
		var resp_collab responses.Collabarator
		mapper.Mapper(&collab, &resp_collab)
		res = append(res, resp_collab)
	}

	w.WriteHeader(http.StatusOK)
	result := utils.StructToJson(res)
	io.WriteString(w, result)
}
