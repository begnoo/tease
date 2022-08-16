package errors

import (
	"RepoService/utils"
	"encoding/json"
	"net/http"
)

type ValidationError struct {
	Err error
}

func (r *ValidationError) Error() string {
	return r.Err.Error()
}

type SameEmailError struct {
	Message string
}

func (r *SameEmailError) Error() string {
	return r.Message
}

type RepoError struct {
	Err error
}

func (r *RepoError) Error() string {
	return r.Err.Error()
}

type MissingEntity struct {
	Message string
}

func (r *MissingEntity) Error() string {
	return r.Message
}

type FailedAuthorization struct {
	Message string
}

func (r *FailedAuthorization) Error() string {
	return r.Message
}

type OwnerMismatch struct {
	Message string
}

func (r *OwnerMismatch) Error() string {
	return r.Message
}

type AlreadyThere struct {
	Message string
}

func (r *AlreadyThere) Error() string {
	return r.Message
}

func NilOrError(err error, req_err error) error {
	if err == nil {
		return nil
	}

	return req_err
}

type ErrorResp struct {
	Error string `json:"error"`
}

func HandleHttpError(req_err error, w http.ResponseWriter) bool {

	if req_err == nil {
		return true
	}

	switch req_err.(type) {
	case *ValidationError:
		{
			w.WriteHeader(http.StatusBadRequest)
			json.NewEncoder(w).Encode(utils.ParseValidationErrToJson(req_err.Error()))
			return false
		}
	case *SameEmailError:
		{
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(ErrorResp{Error: req_err.Error()})
			return false
		}
	case *MissingEntity:
		{
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(ErrorResp{Error: req_err.Error()})
			return false
		}
	case *RepoError:
		{
			w.WriteHeader(http.StatusInternalServerError)
			json.NewEncoder(w).Encode(ErrorResp{Error: req_err.Error()})
			return false
		}
	default:
		w.WriteHeader(http.StatusInternalServerError)
		json.NewEncoder(w).Encode(ErrorResp{Error: req_err.Error()})

		return false
	}
}
