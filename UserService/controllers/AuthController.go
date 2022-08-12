package controllers

import (
	"UserService/di"
	"UserService/errors"
	"UserService/security"
	"encoding/json"
	"net/http"
)

func Login(w http.ResponseWriter, r *http.Request) {
	var creds security.Credidentials
	json.NewDecoder(r.Body).Decode(&creds)
	r.Body.Close()

	err := ValidateStruct(creds)

	if !errors.HandleHttpError(err, w) {
		return
	}

	authService := di.InitializeAuthService()
	token, err := authService.Login(creds)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(token)
}
