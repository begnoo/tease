package controllers

import (
	"UserService/di"
	"UserService/errors"
	"UserService/security"
	"encoding/json"
	"net/http"
)

type TokenResult struct {
	Token string `json:"token"`
}

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
	json.NewEncoder(w).Encode(TokenResult{Token: token})
}

func AccessBackend(w http.ResponseWriter, r *http.Request) {

	token, _ := security.ParseTokenFromRequest(r)

	authService := di.InitializeAuthService()
	new_token, err := authService.GenerateBackendJwt(token)

	if !errors.HandleHttpError(err, w) {
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(new_token)
}
