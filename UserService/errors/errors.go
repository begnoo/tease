package errors

import (
	"UserService/utils"
	"io"
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

func NilOrError(err error, req_err error) error {
	if err == nil {
		return nil
	}

	return req_err
}

func HandleHttpError(req_err error, w http.ResponseWriter) bool {

	if req_err == nil {
		return true
	}

	switch req_err.(type) {
	case *ValidationError:
		{
			w.WriteHeader(http.StatusBadRequest)
			io.WriteString(w, utils.StructToJson(utils.ParseValidationErrToJson(req_err.Error())))
			return false
		}
	case *SameEmailError:
		{
			w.WriteHeader(http.StatusOK)
			io.WriteString(w, req_err.Error())
			return false
		}
	case *MissingEntity:
		{
			w.WriteHeader(http.StatusOK)
			io.WriteString(w, req_err.Error())
			return false
		}
	case *RepoError:
		{
			w.WriteHeader(http.StatusInternalServerError)
			io.WriteString(w, req_err.Error())
			return false
		}
	default:
		w.WriteHeader(http.StatusInternalServerError)
		io.WriteString(w, req_err.Error())
		return false
	}
}
